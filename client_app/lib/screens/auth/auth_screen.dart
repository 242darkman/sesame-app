// ignore_for_file: avoid_unnecessary_containers, prefer_const_constructors

import 'package:client_app/controllers/login_controller.dart';
import 'package:client_app/controllers/register_controller.dart';
import 'package:client_app/widgets/custom_button_widget.dart';
import 'package:client_app/widgets/input_widget.dart';
import 'package:client_app/screens/personnel/ticket_detail.dart';
import 'package:client_app/main.dart';
import 'package:flutter/material.dart';
import 'package:get/get.dart';

class AuthScreen extends StatefulWidget {
  const AuthScreen({super.key});

  @override
  State<AuthScreen> createState() => _AuthScreenState();
}

class _AuthScreenState extends State<AuthScreen> {
  final RegisterationController registerationController =
      Get.put(RegisterationController());
  final LoginController loginController = Get.put(LoginController());
  var isLogin = true.obs;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: SingleChildScrollView(
        child: Padding(
          padding: EdgeInsets.all(36),
          child: Center(
            child: Obx(
              () => Column(
                crossAxisAlignment: CrossAxisAlignment.center,
                mainAxisAlignment: MainAxisAlignment.center,
                children: [
                  SizedBox(height: 30),
                  Container(
                    child: Image.asset(
                      'assets/images/Logo.jpg',
                      width: 300,
                      height: 100,
                      fit: BoxFit.cover,
                    ),
                  ),
                  SizedBox(height: 20),
                  Container(
                    decoration: BoxDecoration(
                      border: Border.all(color: Colors.grey),
                      borderRadius: BorderRadius.circular(5),
                    ),
                    padding: EdgeInsets.all(16),
                    child: Column(
                      children: [
                        Text(
                          isLogin.value ? 'Connexion' : 'Inscription',
                          style: TextStyle(
                            fontSize: 18,
                            fontWeight: FontWeight.bold,
                          ),
                        ),
                        SizedBox(height: 20),
                        isLogin.value ? loginWidget() : registerWidget(),
                      ],
                    ),
                  ),
                  SizedBox(height: 20),
                  CustomButton(
                    onPressed: () => isLogin.value
                        ? loginController.loginWithEmail()
                        : registerationController.registerWithEmail(),
                    title: isLogin.value ? 'Se connecter' : 'S\'inscrire',
                  ),
                  SizedBox(height: 20),
                  GestureDetector(
                    onTap: () {
                      isLogin.value = !isLogin.value;
                    },
                    child: Text(
                      isLogin.value
                          ? 'Pas de compte? S\'inscrire'
                          : 'Déjà un compte? Se connecter',
                      style: TextStyle(
                        color: Colors.blue,
                        decoration: TextDecoration.underline,
                      ),
                    ),
                  ),
                  SizedBox(height: 20),
                  CustomButton(
                    onPressed: () {
                      Navigator.pushNamed(context, '/ticket_detail');
                    },
                    title: 'Voir les détails du ticket',
                  ),
                ],
              ),
            ),
          ),
        ),
      ),
    );
  }

  Widget registerWidget() {
    return Column(
      children: [
        InputTextFieldWidget(registerationController.nameController, 'Nom'),
        SizedBox(height: 20),
        InputTextFieldWidget(registerationController.emailController, 'Email'),
        SizedBox(height: 20),
        InputTextFieldWidget(
            registerationController.passwordController, 'Mot de passe'),
      ],
    );
  }

  Widget loginWidget() {
    return Column(
      children: [
        InputTextFieldWidget(loginController.emailController, 'Email'),
        SizedBox(height: 20),
        InputTextFieldWidget(
            loginController.passwordController, 'Mot de passe'),
      ],
    );
  }
}
