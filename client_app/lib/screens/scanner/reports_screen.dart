import 'package:client_app/widgets/custom_app_bar.dart';
import 'package:client_app/widgets/custom_menu.dart';
import 'package:client_app/models/report_model.dart';
import 'package:client_app/screens/scanner/report_details_screen.dart';
import 'package:client_app/utils/report_functions.dart';
import 'package:flutter/material.dart';

class ReportsScreen extends StatelessWidget {
  const ReportsScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return const Scaffold(
      appBar: CustomAppBar(title: 'Signalements'),
      bottomNavigationBar: CustomMenu(),
      body: ReportList(),
    );
  }
}

class ReportList extends StatefulWidget {
  const ReportList({super.key});

  @override
  _ReportListState createState() => _ReportListState();
}

class _ReportListState extends State<ReportList> {
  int selectedStatusIndex = 0; // Index du statut sélectionné

  // Liste en dur des signalements
  List<Report> reports = [
    Report(
        id: '1',
        status: 'Nouveau',
        date: '17/07/2024',
        type: 'Propreté',
        description: 'lorem ipsum',
        zone: 'Nord',
        location: 'Perrache'),
    Report(
        id: '2',
        status: 'Nouveau',
        date: '18/07/2024',
        type: 'Savon manquant',
        description: 'lorem ipsum',
        zone: 'Sud',
        location: 'Perrache'),
  ];

  void selectStatus(int index) {
    setState(() {
      selectedStatusIndex = index;
    });
  }

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        Padding(
          padding: const EdgeInsets.all(12.0),
          child: Row(
            mainAxisAlignment: MainAxisAlignment.spaceAround,
            children: [
              GestureDetector(
                onTap: () => selectStatus(0),
                child: buildStatus("Nouveau", selectedStatusIndex == 0),
              ),
              GestureDetector(
                onTap: () => selectStatus(1),
                child: buildStatus("En cours", selectedStatusIndex == 1),
              ),
              GestureDetector(
                onTap: () => selectStatus(2),
                child: buildStatus("Résolu", selectedStatusIndex == 2),
              ),
            ],
          ),
        ),
        Container(
          // Barre de separation menu
          height: 1,
          width: double.infinity,
          color: const Color(0xFFA8A8A8),
        ),
        // Liste des cartes
        Expanded(
            child: ListView.builder(
          itemCount: reports.length,
          itemBuilder: (context, index) {
            return reportCard(context, reports[index]);
          },
        )),
      ],
    );
  }

  Widget buildStatus(String text, bool isSelected) {
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 16),
      decoration: BoxDecoration(
        borderRadius: BorderRadius.circular(20),
      ),
      child: Text(
        text,
        style: TextStyle(
          color: isSelected ? const Color(0xFF779DA0) : const Color(0xFF4B4B4B),
        ),
      ),
    );
  }

  Widget reportCard(BuildContext context, Report report) {
    return Card(
      shape: const RoundedRectangleBorder(borderRadius: BorderRadius.zero),
      elevation: 4,
      margin: const EdgeInsets.only(top: 20),
      child: Padding(
        padding: const EdgeInsets.all(20.0),
        child: Row(
          crossAxisAlignment: CrossAxisAlignment.center,
          children: [
            Expanded(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Container(
                    margin: const EdgeInsets.only(bottom: 4),
                    padding: const EdgeInsets.all(8),
                    decoration: BoxDecoration(
                      color: getTypeColor(report.type),
                      borderRadius: BorderRadius.circular(8),
                    ),
                    child: Text(
                      report.type,
                      style: const TextStyle(color: Colors.white),
                    ),
                  ),
                  Padding(
                    padding: const EdgeInsets.symmetric(vertical: 4),
                    child: Text(
                      report.date,
                      style: const TextStyle(fontWeight: FontWeight.bold),
                    ),
                  ),
                  Padding(
                    padding: const EdgeInsets.symmetric(vertical: 4),
                    child: Text("${report.location} ${report.zone}"),
                  ),
                ],
              ),
            ),
            IconButton(
                icon: const Icon(
                  Icons.arrow_forward_ios,
                  color: Color(0xFF4B4B4B),
                  size: 35.0,
                ),
                onPressed: () {
                  Navigator.push(
                    context,
                    MaterialPageRoute(
                        builder: (context) => ReportDetailsScreen()),
                  );
                }),
          ],
        ),
      ),
    );
  }
}
