import 'dart:convert';

import 'package:client_app/main.dart';
import 'package:flutter_secure_storage/flutter_secure_storage.dart';
import 'package:http/http.dart' as http;

class ApiService {
  final String _baseUrl = 'http://localhost:8080';
  final FlutterSecureStorage storage = const FlutterSecureStorage();

  Future<http.Response> createIntervention(
      Map<String, dynamic> interventionData) async {
    try {
      final url = Uri.parse('$_baseUrl/api/intervention');
    final token = await storage.read(key: "keycloak_token");
    final response = await http.post(
      url,
      headers: {
        'Content-Type': 'application/json',
        'Authorization': 'Bearer $token',
      },
      body: jsonEncode(interventionData),
    );

    logger.i('createIntervention response: ${response.body}');
    logger.i('createIntervention response code: ${response.statusCode}');

    if (response.statusCode == 200) {
      return response;
    } else {
      throw Exception('Failed to create intervention: ${response.body}');
    }
    } catch (e) {
      print('Error in createIntervention: $e');
      rethrow;
    }
  }

  Future<List<dynamic>> getInterventionsWithComments() async {
    final url = Uri.parse('$_baseUrl/intervention');
    final token = await storage.read(key: "keycloak_token");
    final response = await http.get(
      url,
      headers: {
        'Content-Type': 'application/json',
        'Authorization': 'Bearer $token',
      },
    );

    if (response.statusCode == 200) {
      return jsonDecode(response.body)['interventions'];
    } else {
      throw Exception('Failed to load interventions');
    }
  }
}
