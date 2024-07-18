import 'package:flutter/material.dart';
import 'package:client_app/widgets/custom_app_bar.dart';
import 'package:client_app/widgets/custom_menu.dart';

class ReportDetailsScreen extends StatelessWidget {
  const ReportDetailsScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Flutter Button Example',
      theme: ThemeData(
        primarySwatch: Colors.blue,
      ),
      home: DetailScreen(),
    );
  }
}

class DetailScreen extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: CustomAppBar(title: 'Détails du signalement'),
      body: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            Center(
              child: ElevatedButton(
                style: ElevatedButton.styleFrom(
                  backgroundColor: Color(0xFFD1953A), // Couleur brune
                  foregroundColor: Colors.white, // Texte blanc
                  shape: RoundedRectangleBorder(
                    borderRadius: BorderRadius.circular(5), // Coins arrondis
                  ),
                  padding: EdgeInsets.symmetric(
                      horizontal: 16, vertical: 8), // Réduire le padding
                  minimumSize: Size(100, 40), // Ajuster la taille du bouton
                ),
                onPressed: () {
                  // Action lors de l'appui sur le bouton
                  print('NOUVEAU Button pressed');
                  Navigator.push(
                    context,
                    MaterialPageRoute(
                        builder: (context) => TicketDetailScreen()),
                  );
                },
                child: Text('NOUVEAU'),
              ),
            ),
            /*    SizedBox(height: 10), // Espacement entre le bouton et les étoiles
            Center(
              child: Row(
                mainAxisSize: MainAxisSize.min,
                children: [
                  Icon(Icons.star, color: Colors.yellow, size: 30),
                  Icon(Icons.star, color: Colors.yellow, size: 30),
                ],
              ),
            ), */
            SizedBox(height: 20), // Ajouter de l'espace entre les éléments
            Align(
              alignment: Alignment.centerLeft,
              child: ElevatedButton(
                style: ElevatedButton.styleFrom(
                  backgroundColor: Color(0xFF30959B), // Couleur du bouton
                  foregroundColor: Colors.white, // Texte blanc
                  shape: RoundedRectangleBorder(
                    borderRadius: BorderRadius.circular(5), // Coins arrondis
                  ),
                  padding: EdgeInsets.symmetric(
                      horizontal: 16, vertical: 8), // Réduire le padding
                  minimumSize: Size(100, 40), // Ajuster la taille du bouton
                ),
                onPressed: () {
                  // Action lors de l'appui sur le bouton
                  print('Button above Date pressed');
                },
                child: Text('Propreté'),
              ),
            ),
            SizedBox(height: 20), // Espacement entre le bouton et la date
            InfoRow(
              label: 'Date:',
              value: '15/07/2024',
            ),
            SizedBox(height: 10),
            InfoRow(
              label: 'Localisation:',
              value: 'Paris, France',
            ),
            SizedBox(height: 20), // Ajouter de l'espace avant la description
            Text(
              'Description :',
              style: TextStyle(fontWeight: FontWeight.bold),
            ),
            SizedBox(height: 10),
            Text(
              'Lorem ipsum dolor sit amet, consectetur adipiscing elit. Vivamus lacinia odio vitae vestibulum vestibulum. Cras venenatis euismod malesuada.',
            ),
            SizedBox(height: 20), // Ajouter de l'espace avant les boutons
            ElevatedButton(
              style: ElevatedButton.styleFrom(
                backgroundColor: Color(0xFF7D949B), // Couleur du bouton
                foregroundColor: Colors.white, // Texte blanc
                shape: RoundedRectangleBorder(
                  borderRadius: BorderRadius.circular(5), // Coins arrondis
                ),
                padding: EdgeInsets.symmetric(
                    horizontal: 16, vertical: 8), // Réduire le padding
                minimumSize: Size(double.infinity, 40), // Bouton pleine largeur
              ),
              onPressed: () {
                // Action lors de l'appui sur le bouton
                print('Commentaires Button pressed');
                Navigator.push(
                  context,
                  MaterialPageRoute(
                      builder: (context) =>
                          CommentsScreen()), // Navigue vers la page des commentaires
                );
              },
              child: Text('Commentaires'),
            ),
            SizedBox(height: 10), // Ajouter de l'espace entre les boutons
            ElevatedButton(
              style: ElevatedButton.styleFrom(
                backgroundColor: Color(0xFF30959B), // Couleur du bouton
                foregroundColor: Colors.white, // Texte blanc
                shape: RoundedRectangleBorder(
                  borderRadius: BorderRadius.circular(5), // Coins arrondis
                ),
                padding: EdgeInsets.symmetric(
                    horizontal: 16, vertical: 8), // Réduire le padding
                minimumSize: Size(double.infinity, 40), // Bouton pleine largeur
              ),
              onPressed: () {
                // Action lors de l'appui sur le bouton
                print('En cours Button pressed');
                Navigator.push(
                  context,
                  MaterialPageRoute(builder: (context) => TicketDetailScreen()),
                );
              },
              child: Text('En cours'),
            ),
          ],
        ),
      ),
      bottomNavigationBar: CustomMenu(),
    );
  }
}

class InfoRow extends StatelessWidget {
  final String label;
  final String value;

  InfoRow({required this.label, required this.value});

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Row(
          children: [
            Text(
              label,
              style: TextStyle(fontWeight: FontWeight.bold),
            ),
            SizedBox(width: 8),
            Expanded(
              child: Text(
                value,
                style: TextStyle(fontWeight: FontWeight.normal),
              ),
            ),
          ],
        ),
        Divider(
          color: Colors.black,
          thickness: 0.3, // Définir l'épaisseur du Divider
        ),
      ],
    );
  }
}

class TicketDetailScreen extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text('Ticket Detail'),
      ),
      body: Center(
        child: Text('This is the ticket detail screen'),
      ),
    );
  }
}

class CommentsScreen extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text('Comments'),
      ),
      body: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Text('This is the comments screen'),
      ),
    );
  }
}
