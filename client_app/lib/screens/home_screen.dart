import 'package:client_app/main.dart';
import 'package:flutter/material.dart';

class HomeScreen extends StatelessWidget {
  const HomeScreen({super.key});

  // Logout from the current realm.
  Future<bool> logout() async {
    // Check if user has successfully logged out.
    final isLoggedOut = await keycloakWrapper.logout();

    return isLoggedOut;
  }

  @override
  Widget build(BuildContext context) => Scaffold(
        body: Center(
          child: Column(
            children: [
              FutureBuilder(
                // Retrieve the user information.
                future: keycloakWrapper.getUserInfo(),
                builder: (context, snapshot) {
                  final name = snapshot.data?['name'] as String;
                  final email = snapshot.data?['email'] as String;

                  return Text('$name\n$email\n\n');
                },
              ),
              TextButton(onPressed: logout, child: const Text('Logout')),
            ],
          ),
        ),
      );
}
