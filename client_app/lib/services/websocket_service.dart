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

  List<dynamic> _locations = [];
  List<dynamic> _zones = [];
  List<dynamic> _types = [];

  List<dynamic> get locations => _locations;
  List<dynamic> get zones => _zones;
  List<dynamic> get types => _types;

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
        _handleMessage(message);
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

  /// Gère les messages reçus du serveur WebSocket.
  ///
  /// Cette méthode décode le message JSON et met à jour les variables
  /// appropriées en fonction du contenu du message.
  void _handleMessage(String message) {
    final decodedMessage = jsonDecode(message);
    logger.i("decodedMessage: $decodedMessage");

    if (decodedMessage is Map<String, dynamic>) {
      if (decodedMessage.containsKey('locations')) {
        _locations = decodedMessage['locations'];
        logger.i("Locations: $_locations");
      } else if (decodedMessage.containsKey('zones')) {
        _zones = decodedMessage['zones'];
        logger.i("Zones: $_zones");
      } else if (decodedMessage.containsKey('problem_types')) {
        _types = decodedMessage['problem_types'];
        logger.i("Types: $_types");
      } else if (decodedMessage.containsKey('status') &&
          decodedMessage['status'] == 'Ok') {
        notifyListeners();
      } else {
        _message = message;
      }
    } else {
      _message = message;
    }

    notifyListeners();
    logger.i("Reçu: $message");
  }

  /// Envoie un message au serveur WebSocket.
  ///
  /// Utilise le canal WebSocket pour envoyer le [message] fourni.
  void sendMessage(String message) {
    _channel?.sink.add(message);
  }

  /// Demande la liste des emplacements au serveur WebSocket.
  ///
  /// Envoie une requête pour obtenir les emplacements.
  void requestLocations() {
    _channel?.sink.add("REQUEST_LOCATIONS");
  }

  /// Demande la liste des zones au serveur WebSocket.
  void requestZones() {
    _channel?.sink.add("REQUEST_ZONES");
  }

  /// Demande la liste des types au serveur WebSocket.
  void requestTypes() {
    _channel?.sink.add("REQUEST_PROBLEM_TYPES");
  }

  // Method to create a new intervention
  void createIntervention(
      Map<String, dynamic> interventionData, String userId) {
    /*final message = jsonEncode({
      "user_id": userId,
      "type": "CREATE_INTERVENTION",
      "data": interventionData,
      "message": "Action de création d'intervention"
    });*/

    _channel?.sink.add(jsonEncode({
      "user_id": userId,
      "type": "CREATE_INTERVENTION",
      "data": interventionData,
      "message": "Action de création d'intervention"
    }));
    //sendMessage(message);
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
