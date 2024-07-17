import 'package:flutter/material.dart';
import 'package:jwt_decoder/jwt_decoder.dart';

class UserProvider with ChangeNotifier {
  String? _accessToken;
  Map<String, dynamic>? _user;

  String? get accessToken => _accessToken;
  Map<String, dynamic>? get user => _user;

  void setUserInfo(
      {required String? accessToken, required Map<String, dynamic>? user}) {
    _accessToken = accessToken;
    _user = user;
    notifyListeners();
  }

  void clearUserInfo() {
    _accessToken = null;
    _user = null;
    notifyListeners();
  }

  bool isTokenExpired() {
    if (_accessToken == null) return true;
    return JwtDecoder.isExpired(_accessToken!);
  }
}
