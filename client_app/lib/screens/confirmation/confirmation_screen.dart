import 'package:client_app/widgets/custom_app_bar.dart';
import 'package:client_app/widgets/custom_menu.dart';
import 'package:flutter/material.dart';

class ConfirmationScreen extends StatelessWidget {
  const ConfirmationScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return const Scaffold(
      appBar: CustomAppBar(title: 'Signaler un problème'),
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            Text(
              'Merci',
              style: TextStyle(
                fontSize: 24,
                fontWeight: FontWeight.bold,
                color: Color(0xff4b4b4b),
              ),
            ),
            SizedBox(height: 20),
            Text(
              'Votre signalement a bien été envoyé',
              style: TextStyle(
                fontSize: 16,
                color: Color(0xff4b4b4b),
              ),
              textAlign: TextAlign.center,
            ),
            SizedBox(height: 20),
            Icon(
              Icons.check,
              color: Colors.green,
              size: 50,
            ),
          ],
        ),
      ),
      bottomNavigationBar: CustomMenu(),
    );
  }
}
