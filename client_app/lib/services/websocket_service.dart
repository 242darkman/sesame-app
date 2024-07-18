import 'dart:convert';

import 'package:client_app/utils/env.dart';
import 'package:flutter/material.dart';
import 'package:flutter_secure_storage/flutter_secure_storage.dart';
import 'package:logger/logger.dart';
import 'package:web_socket_channel/io.dart';
import 'package:web_socket_channel/status.dart' as status;

/// Une classe de service qui gère les connexions WebSocket.
///
/// Cette classe gère la connexion, la messagerie et la déconnexion des
/// connexions WebSocket, ainsi que la gestion des messages reçus et la
/// journalisation.
class WebSocketService with ChangeNotifier {
  /// Stockage sécurisé pour stocker et récupérer des données sensibles.
  final FlutterSecureStorage storage = const FlutterSecureStorage();

  /// Canal WebSocket utilisé pour la communication.
  IOWebSocketChannel? _channel;

  /// Logger pour enregistrer les informations, avertissements et erreurs.
  final Logger logger = Logger();

  /// Le dernier message reçu du WebSocket.
  String? _message;

  /// Getter pour le dernier message.
  String? get message => _message;

  /// Se connecte au serveur WebSocket en utilisant l'[userId] fourni.
  ///
  /// Récupère le jeton Keycloak depuis le stockage sécurisé et établit une
  /// connexion WebSocket au serveur. Écoute les messages, erreurs et
  /// la fermeture de la connexion.
  Future<void> connect(String userId) async {
    final token = await storage.read(key: "keycloak_token");
    final headers = {'Authorization': 'Bearer $token'};
    logger.i("token ws $token");

    if (token != null) {
      final url =
          "ws://${Environment.API_URL.replaceFirst('http://', '')}api/v1/ws/$userId";

      logger.i("Connecting to WebSocket URL: $url");

      _channel = IOWebSocketChannel.connect(Uri.parse(url), headers: headers);

      _channel!.stream.listen((message) {
        _message = message;
        notifyListeners();
        logger.i("Reçu: $message");
      }, onError: (error) {
        logger.e("Erreur: $error");
      }, onDone: () {
        logger.i("WebSocket fermé");
      });

      logger.i("Sending initial message to WebSocket.");
      _channel!.sink.add(jsonEncode(
          {"user_id": userId, "message": "Bonjour depuis le client Flutter!"}));
    } else {
      logger.w("Aucun jeton Keycloak trouvé");
    }
  }

  /// Envoie un message au serveur WebSocket.
  ///
  /// Utilise le canal WebSocket pour envoyer le [message] fourni.
  void sendMessage(String message) {
    _channel?.sink.add(message);
  }

  /// Se déconnecte du serveur WebSocket.
  ///
  /// Ferme la connexion WebSocket avec un statut 'going away'.
  void disconnect() {
    _channel?.sink.close(status.goingAway);
  }

  @override
  void dispose() {
    disconnect();
    super.dispose();
  }
}
