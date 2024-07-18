import 'package:flutter/material.dart';
import 'package:client_app/widgets/custom_app_bar.dart';
import 'package:client_app/widgets/custom_menu.dart';

class ProblemScreen extends StatelessWidget {
  const ProblemScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return const Scaffold(
      appBar: const CustomAppBar(title: 'Signaler un problème'),
      bottomNavigationBar: const CustomMenu(),
      body: ProblemForm(),
    );
  }
}

class ProblemForm extends StatefulWidget {
  const ProblemForm({super.key});

  @override
  _ProblemFormState createState() => _ProblemFormState();
}

class _ProblemFormState extends State<ProblemForm> {
  String? _selectedType;

  @override
  Widget build(BuildContext context) {
    return Container(
      color: Colors.white,
      child: Padding(
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
      ),
    );
  }
}
