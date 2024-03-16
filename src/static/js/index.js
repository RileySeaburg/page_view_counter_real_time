document.addEventListener('DOMContentLoaded', function() {
    const activeUsersElement = document.getElementById('activeUsers');
    const wsScheme = window.location.protocol === 'https:' ? 'wss' : 'ws';
    const wsPath = `${wsScheme}://${window.location.host}/ws`;

    const socket = new WebSocket(wsPath);

    socket.onmessage = function(event) {
        const message = event.data;
        if (message.startsWith('User count: ')) {
            activeUsersElement.textContent = message.replace('User count: ', '');
        }
    };
    

    socket.onerror = function(error) {
        console.log('WebSocket Error: ' + error);
    };

    // You can also handle socket.onopen and socket.onclose as needed
});
