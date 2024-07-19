import 'package:client_app/provider/user_provider.dart';
import 'package:client_app/services/api_service.dart';
import 'package:client_app/services/websocket_service.dart';
import 'package:client_app/widgets/custom_app_bar.dart';
import 'package:client_app/widgets/custom_menu.dart';
import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import 'package:provider/provider.dart';

class ProblemWithoutScanScreen extends StatelessWidget {
  const ProblemWithoutScanScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return const Scaffold(
      appBar: CustomAppBar(title: 'Signaler un problème'),
      bottomNavigationBar: CustomMenu(),
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
  final TextEditingController _commentController = TextEditingController();

  @override
  void initState() {
    super.initState();
    final webSocketService =
        Provider.of<WebSocketService>(context, listen: false);
    webSocketService.requestLocations();
    webSocketService.requestZones();
    webSocketService.requestTypes();
  }

  @override
  Widget build(BuildContext context) {
    return Consumer<WebSocketService>(
      builder: (context, webSocketService, child) {
        final isLoading = webSocketService.locations.isEmpty ||
            webSocketService.zones.isEmpty ||
            webSocketService.types.isEmpty;

        return Container(
          color: Colors.white,
          padding: const EdgeInsets.all(20.0),
          child: isLoading
              ? const Center(child: CircularProgressIndicator())
              : SingleChildScrollView(
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
                          items: webSocketService.locations
                              .map<DropdownMenuItem<String>>((location) {
                            final name = location['name'] ?? 'N/A';
                            return DropdownMenuItem<String>(
                              value: name,
                              child: Text(name),
                            );
                          }).toList(),
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
                          items: webSocketService.zones
                              .map<DropdownMenuItem<String>>((zone) {
                            final name = zone['name'] ?? 'N/A';
                            return DropdownMenuItem<String>(
                              value: name,
                              child: Text(name),
                            );
                          }).toList(),
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
                          items: webSocketService.types
                              .map<DropdownMenuItem<String>>((type) {
                            final name = type['defaulttype'] ?? 'N/A';
                            return DropdownMenuItem<String>(
                              value: name,
                              child: Text(name),
                            );
                          }).toList(),
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
                          controller: _commentController,
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
                          onPressed: () async {
                            final webSocketService =
                                Provider.of<WebSocketService>(context,
                                    listen: false);
                            final userProvider = Provider.of<UserProvider>(
                                context,
                                listen: false);
                            final apiService = ApiService();

                            final selectedLocation = webSocketService.locations
                                .firstWhere((location) =>
                                    location['name'] == _selectedEmplacement);
                            final selectedType = webSocketService.types
                                .firstWhere((type) =>
                                    type['defaulttype'] == _selectedType);

                            final newIntervention = {
                              "iduser": userProvider.user?['sub'],
                              "description": null,
                              "iddefault": selectedType['id'],
                              "idlocation": selectedLocation['id'],
                              "comment": _commentController.text,
                            };

                            final response = await apiService
                                .createIntervention(newIntervention);
                            if (response.statusCode == 200) {
                              // Handle successful intervention creation
                              if (context.mounted) {
                                context.go(
                                    '/app/scanner/report_comment/confirmation');
                              }
                            } else {
                              // Handle error in intervention creation
                              if (context.mounted) {
                                ScaffoldMessenger.of(context).showSnackBar(
                                  const SnackBar(
                                    content:
                                        Text('Failed to create intervention'),
                                  ),
                                );
                              }
                            }
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
      },
    );
  }
}
