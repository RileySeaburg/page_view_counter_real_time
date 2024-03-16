use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use actix_web::{web, App, HttpServer, get, HttpRequest, Error, HttpResponse};
use actix_web_actors::ws;
use actix::{Actor, StreamHandler, Message, Handler, Addr, ActorContext};
use actix::AsyncContext;

struct AppState {
    active_users: AtomicUsize,
    active_sockets: Mutex<Vec<Addr<UserSocket>>>
}

struct UserSocket {
    app_state: web::Data<AppState>,
}

impl Actor for UserSocket {
    type Context = ws::WebsocketContext<Self>;
}


impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for UserSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }

    fn started(&mut self, ctx: &mut Self::Context) {
        
        self.app_state.active_users.fetch_add(1, Ordering::SeqCst);
        self.app_state.active_sockets.lock().unwrap().push(ctx.address()); // Add this line

        for socket in self.app_state.active_sockets.lock().unwrap().iter() {
            socket.do_send(UserCount(self.app_state.active_users.load(Ordering::SeqCst)));
        }
    }

    fn finished(&mut self, ctx: &mut Self::Context) {
        
        self.app_state.active_users.fetch_sub(1, Ordering::SeqCst);
        let socket_index = self.app_state.active_sockets.lock().unwrap().iter().position(|x| *x == ctx.address()).unwrap();
        self.app_state.active_sockets.lock().unwrap().remove(socket_index);

        for socket in self.app_state.active_sockets.lock().unwrap().iter() {
            socket.do_send(UserCount(self.app_state.active_users.load(Ordering::SeqCst)));
        }
    }
}



struct UserCount(usize);

impl Message for UserCount {
    type Result = ();
}

impl Handler<UserCount> for UserSocket {
    type Result = ();

    fn handle(&mut self, msg: UserCount, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(format!("User count: {}", msg.0));
    }
}

#[get("/")]
async fn index(data: web::Data<AppState>, tmpl: web::Data<tera::Tera>) -> HttpResponse {
    let active_users = data.active_users.load(Ordering::SeqCst);
    format!("Active users: {}", active_users);
    // serve index.html
   
   let mut ctx = tera::Context::new();
    ctx.insert("active_users", &active_users);

    let rendered = tmpl.render("index.html.tera", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)

}

#[get("/2")]
async fn index2(data: web::Data<AppState>, tmpl: web::Data<tera::Tera>) -> HttpResponse {
    let active_users = data.active_users.load(Ordering::SeqCst);
    format!("Active users: {}", active_users);
    // serve index.html
   
   let mut ctx = tera::Context::new();
    ctx.insert("active_users", &active_users);

    let rendered = tmpl.render("index2.html.tera", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

 async fn ws_index(req: HttpRequest, stream: web::Payload, app_state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let user_socket = UserSocket {
        app_state: app_state.clone(),
    };
    
    ws::start(user_socket, &req, stream)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {



    let app_state = web::Data::new(AppState {
        active_users: AtomicUsize::new(0),
        active_sockets: Mutex::new(Vec::new())
    });


    
    HttpServer::new(move || {
        let tera = tera::Tera::new("templates/**/*").unwrap();



        App::new()
            .app_data(web::Data::new(tera))
            .app_data(app_state.clone())
            .service(index)
            .route("/ws", web::get().to(ws_index))
            // serviec static files
            .service(actix_files::Files::new("/", "./src/static/"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}