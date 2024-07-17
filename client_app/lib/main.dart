import 'package:flutter/material.dart';
import 'package:get/get.dart';
import 'package:client_app/screens/auth/auth_screen.dart';
import 'package:client_app/screens/personnel/ticket_detail.dart';

void main() {
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return GetMaterialApp(
      title: 'Sesame',
      debugShowCheckedModeBanner: false,
      home: AuthScreen(),
      getPages: [
        GetPage(name: '/', page: () => AuthScreen()),
        GetPage(name: '/ticket_detail', page: () => DetailApp()),
      ],
    );
  }
}
