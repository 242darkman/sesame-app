import 'package:flutter/material.dart';
import 'package:flutter_secure_storage/flutter_secure_storage.dart';
import 'package:logger/logger.dart';
import 'package:web_socket_channel/io.dart';
import 'package:web_socket_channel/status.dart' as status;

class WebSocketService with ChangeNotifier {
  final FlutterSecureStorage storage = const FlutterSecureStorage();
  IOWebSocketChannel? _channel;
  final Logger logger = Logger();
  String? _message;
  String? get message => _message;

  Future<void> connect() async {
    final token = await storage.read(key: "keycloak_token");
    if (token != null) {
      const userId = "your-user-id"; // Retrieve this dynamically if possible
      final url = "ws://your_server_ip:8080/ws/$userId?token=$token";

      _channel = IOWebSocketChannel.connect(Uri.parse(url));

      _channel!.stream.listen((message) {
        _message = message;
        notifyListeners();
        logger.i("Received: $message");
      }, onError: (error) {
        logger.e("Error: $error");
      }, onDone: () {
        logger.i("WebSocket closed");
      });

      _channel!.sink.add("Hello from Flutter client!");
    } else {
      logger.w("No Keycloak token found");
    }
  }

  void sendMessage(String message) {
    _channel?.sink.add(message);
  }

  void disconnect() {
    _channel?.sink.close(status.goingAway);
  }

  @override
  void dispose() {
    disconnect();
    super.dispose();
  }
}
