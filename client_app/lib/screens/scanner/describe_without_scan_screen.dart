import 'package:flutter/material.dart';
import 'package:client_app/widgets/custom_app_bar.dart';
import 'package:client_app/widgets/custom_menu.dart';

class ProblemWithoutScanScreen extends StatelessWidget {
  const ProblemWithoutScanScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return const Scaffold(
      appBar: const CustomAppBar(title: 'Signaler un problème'),
      bottomNavigationBar: const CustomMenu(),
      body: ProblemFormWithoutScan(),
    );
  }
}

class ProblemFormWithoutScan extends StatefulWidget {
  const ProblemFormWithoutScan({super.key});

  @override
  _ProblemFormWithoutScanState createState() => _ProblemFormWithoutScanState();
}

class _ProblemFormWithoutScanState extends State<ProblemFormWithoutScan> {
  String? _selectedEmplacement;
  String? _selectedZone;
  String? _selectedType;

  @override
  Widget build(BuildContext context) {
    return Container(
      color: Colors.white,
      padding: const EdgeInsets.all(20.0),
      child: SingleChildScrollView(
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            const Center(
              child: Text(
                'Décrivez le problème',
                style: TextStyle(
                  fontSize: 22,
                  fontWeight: FontWeight.normal,
                  color: Color(0xff4b4b4b),
                ),
              ),
            ),
            const SizedBox(height: 50),
            const Text(
              'Emplacement :',
              style: TextStyle(
                fontSize: 18,
                fontWeight: FontWeight.bold,
              ),
            ),
            Container(
              decoration: const BoxDecoration(
                boxShadow: [
                  BoxShadow(
                    color: Colors.black12,
                    blurRadius: 4,
                    offset: Offset(0, 2),
                  ),
                ],
              ),
              child: DropdownButtonFormField<String>(
                decoration: InputDecoration(
                  border: OutlineInputBorder(
                    borderRadius: BorderRadius.circular(5),
                    borderSide: BorderSide.none,
                  ),
                  filled: true,
                  fillColor: Colors.white,
                ),
                value: _selectedEmplacement,
                items: const [
                  DropdownMenuItem(
                    value: 'Part dieu',
                    child: Text('Part dieu'),
                  ),
                  DropdownMenuItem(
                    value: 'Confluence',
                    child: Text('Confluence'),
                  ),
                  DropdownMenuItem(
                    value: 'Carré de soie',
                    child: Text('Carré de soie'),
                  ),
                ],
                onChanged: (value) {
                  setState(() {
                    _selectedEmplacement = value;
                  });
                },
                hint: const Text(
                  'Sélectionner un Emplacement',
                  style: TextStyle(
                    color: Colors.grey,
                  ),
                ),
              ),
            ),
            const SizedBox(height: 20),
            const Text(
              'Zone :',
              style: TextStyle(
                fontSize: 18,
                fontWeight: FontWeight.bold,
              ),
            ),
            Container(
              decoration: const BoxDecoration(
                boxShadow: [
                  BoxShadow(
                    color: Colors.black12,
                    blurRadius: 4,
                    offset: Offset(0, 2),
                  ),
                ],
              ),
              child: DropdownButtonFormField<String>(
                decoration: InputDecoration(
                  border: OutlineInputBorder(
                    borderRadius: BorderRadius.circular(5),
                    borderSide: BorderSide.none,
                  ),
                  filled: true,
                  fillColor: Colors.white,
                ),
                value: _selectedZone,
                items: const [
                  DropdownMenuItem(
                    value: 'Zone A',
                    child: Text('Zone A'),
                  ),
                  DropdownMenuItem(
                    value: 'Zone B',
                    child: Text('Zone B'),
                  ),
                  DropdownMenuItem(
                    value: 'Zone C',
                    child: Text('Zone C'),
                  ),
                ],
                onChanged: (value) {
                  setState(() {
                    _selectedZone = value;
                  });
                },
                hint: const Text(
                  'Sélectionner une zone',
                  style: TextStyle(
                    color: Colors.grey,
                  ),
                ),
              ),
            ),
            const SizedBox(height: 20),
            const Text(
              'Type :',
              style: TextStyle(
                fontSize: 18,
                fontWeight: FontWeight.bold,
              ),
            ),
            Container(
              decoration: const BoxDecoration(
                boxShadow: [
                  BoxShadow(
                    color: Colors.black12,
                    blurRadius: 4,
                    offset: Offset(0, 2),
                  ),
                ],
              ),
              child: DropdownButtonFormField<String>(
                decoration: InputDecoration(
                  border: OutlineInputBorder(
                    borderRadius: BorderRadius.circular(5),
                    borderSide: BorderSide.none,
                  ),
                  filled: true,
                  fillColor: Colors.white,
                ),
                value: _selectedType,
                items: const [
                  DropdownMenuItem(
                    value: 'Propreté',
                    child: Text('Propreté'),
                  ),
                  DropdownMenuItem(
                    value: 'État des installations',
                    child: Text('État des installations'),
                  ),
                ],
                onChanged: (value) {
                  setState(() {
                    _selectedType = value;
                  });
                },
                hint: const Text(
                  'Sélectionner un type',
                  style: TextStyle(
                    color: Colors.grey,
                  ),
                ),
              ),
            ),
            const SizedBox(height: 20),
            const Text(
              'Commentaire :',
              style: TextStyle(
                fontSize: 18,
                fontWeight: FontWeight.bold,
              ),
            ),
            Container(
              decoration: const BoxDecoration(
                boxShadow: [
                  BoxShadow(
                    color: Colors.black12,
                    blurRadius: 4,
                    offset: Offset(0, 2),
                  ),
                ],
              ),
              child: TextFormField(
                maxLines: 4,
                decoration: InputDecoration(
                  hintText: 'Donner des informations supplémentaires',
                  hintStyle: TextStyle(
                    color: Colors.grey[400],
                  ),
                  border: OutlineInputBorder(
                    borderRadius: BorderRadius.circular(5),
                    borderSide: BorderSide.none,
                  ),
                  filled: true,
                  fillColor: Colors.white,
                ),
              ),
            ),
            const SizedBox(height: 50),
            SizedBox(
              width: double.infinity,
              child: ElevatedButton(
                onPressed: () {
                  // Action à effectuer lorsque le bouton est pressé
                },
                style: ElevatedButton.styleFrom(
                  backgroundColor: const Color(0xff30959b),
                  shape: RoundedRectangleBorder(
                    borderRadius: BorderRadius.circular(5),
                  ),
                ),
                child: const Text(
                  'Envoyer',
                  style: TextStyle(
                    color: Colors.white,
                    fontSize: 16,
                  ),
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }
}
