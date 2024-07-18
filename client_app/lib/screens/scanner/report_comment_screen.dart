import 'package:flutter/material.dart';
import 'package:client_app/widgets/custom_app_bar.dart';
import 'package:client_app/widgets/custom_menu.dart';

class ReportCommentScreen extends StatelessWidget {
  const ReportCommentScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Comments Example',
      theme: ThemeData(
        primarySwatch: Colors.blue,
      ),
      home: CommentsScreen(),
    );
  }
}

class CommentsScreen extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: CustomAppBar(title: 'Commentaires'),
      body: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            Text(
              'User',
              style: TextStyle(fontWeight: FontWeight.bold),
            ),
            SizedBox(height: 8),
            Text(
              'Erica N.', // Exemple de date à afficher dynamiquement
              style: TextStyle(fontSize: 16),
            ),
            SizedBox(height: 16),
            Divider(
              color: Colors.black,
              thickness: 0.3,
            ),
            SizedBox(height: 16),
            Text(
              'Description:',
              style: TextStyle(fontWeight: FontWeight.bold),
            ),
            SizedBox(height: 8),
            Text(
              'Lorem ipsum dolor sit amet, consectetur adipiscing elit. '
              'Nullam finibus ex sit amet risus convallis, eu eleifend '
              'purus placerat. Vivamus ut leo ut odio iaculis sodales eu eu justo.',
              style: TextStyle(fontSize: 16),
            ),
            SizedBox(height: 16),
            Text(
              'Votre commentaire : ',
              style: TextStyle(fontWeight: FontWeight.bold),
            ),
            SizedBox(height: 16),
            Container(
              decoration: BoxDecoration(
                color: Colors.white,
                borderRadius: BorderRadius.circular(10),
                boxShadow: [
                  BoxShadow(
                    color: Colors.grey.withOpacity(0.5),
                    spreadRadius: 5,
                    blurRadius: 7,
                    offset: Offset(4, 3),
                  ),
                ],
              ),
              child: TextField(
                maxLines: 5,
                decoration: InputDecoration(
                  hintText: 'Donnez des informations supplémentaires...',
                  contentPadding: EdgeInsets.all(16),
                  border: OutlineInputBorder(
                    borderRadius: BorderRadius.circular(10),
                    borderSide: BorderSide.none,
                  ),
                ),
              ),
            ),
            SizedBox(height: 10), // Add space between the two buttons
            ElevatedButton(
              style: ElevatedButton.styleFrom(
                backgroundColor: Color(0xff30959b), // Button color
                foregroundColor: Colors.white, // White text
                shape: RoundedRectangleBorder(
                  borderRadius: BorderRadius.circular(5), // Rounded corners
                ),
                padding: EdgeInsets.symmetric(
                    horizontal: 16, vertical: 8), // Reduce padding
                minimumSize: Size(double.infinity, 40), // Full width button
              ),
              onPressed: () {
                // Action when button is pressed
                print('En cours Button pressed');
                Navigator.push(
                  context,
                  MaterialPageRoute(builder: (context) => CommentsScreen()),
                );
              },
              child: Text('Ajouter'),
            ),
          ],
        ),
      ),
      bottomNavigationBar: CustomMenu(),
    );
  }
}
