// ignore_for_file: avoid_unnecessary_containers, prefer_const_constructors

import 'package:client_app/controllers/login_controller.dart';
import 'package:client_app/controllers/register_controller.dart';
import 'package:client_app/widgets/custom_button_widget.dart';
import 'package:client_app/widgets/input_widget.dart';
import 'package:flutter/material.dart';
import 'package:get/get.dart';

class AuthScreen extends StatefulWidget {
  const AuthScreen({super.key});

  @override
  State<AuthScreen> createState() => _AuthScreenState();
}

class _AuthScreenState extends State<AuthScreen> {
  RegisterationController registerationController =
      Get.put(RegisterationController());

  LoginController loginController = Get.put(LoginController());

  var isLogin = false.obs;
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
                    SizedBox(
                      height: 30,
                    ),
                    Container(
                      child: Text(
                        'Challenge stack',
                        style: TextStyle(
                            fontSize: 30,
                            color: Colors.black,
                            fontWeight: FontWeight.w400),
                      ),
                    ),
                    SizedBox(
                      height: 20,
                    ),
                    Row(
                      mainAxisAlignment: MainAxisAlignment.center,
                      children: [
                        MaterialButton(
                          color: !isLogin.value
                              ? Colors.white
                              : Colors.blue.shade800,
                          onPressed: () {
                            isLogin.value = false;
                          },
                          child: Text(
                            'Inscription',
                            style: TextStyle(
                              color:
                                  !isLogin.value ? Colors.black : Colors.white,
                              fontWeight: FontWeight.bold,
                            ),
                          ),
                        ),
                        MaterialButton(
                          color: isLogin.value
                              ? Colors.white
                              : Colors.blue.shade800,
                          onPressed: () {
                            isLogin.value = true;
                          },
                          child: Text(
                            'Connexion',
                            style: TextStyle(
                              color:
                                  isLogin.value ? Colors.black : Colors.white,
                              fontWeight: FontWeight.bold,
                            ),
                          ),
                        ),
                      ],
                    ),
                    SizedBox(
                      height: 80,
                    ),
                    isLogin.value ? loginWidget() : registerWidget()
                  ]),
            ),
          ),
        ),
      ),
    );
  }

  Widget registerWidget() {
    return Column(
      children: [
        InputTextFieldWidget(registerationController.nameController, 'name'),
        SizedBox(
          height: 20,
        ),
        InputTextFieldWidget(registerationController.emailController, 'Email'),
        SizedBox(
          height: 20,
        ),
        InputTextFieldWidget(
            registerationController.passwordController, 'Mot de passe'),
        SizedBox(
          height: 20,
        ),
        CustomButton(
          onPressed: () => registerationController.registerWithEmail(),
          title: 'S\'inscrire',
        )
      ],
    );
  }

  Widget loginWidget() {
    return Column(
      children: [
        SizedBox(
          height: 20,
        ),
        InputTextFieldWidget(loginController.emailController, 'Email'),
        SizedBox(
          height: 20,
        ),
        InputTextFieldWidget(
            loginController.passwordController, 'Mot de passe'),
        SizedBox(
          height: 20,
        ),
        CustomButton(
          onPressed: () => loginController.loginWithEmail(),
          title: 'Se connecter',
        )
      ],
    );
  }
}
