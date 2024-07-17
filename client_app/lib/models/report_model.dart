class Report {
  String id;
  String status;
  String date;
  String type;
  String description;
  String zone;
  String location;

  Report({
    required this.id,
    required this.status,
    required this.date,
    required this.type,
    required this.description,
    required this.zone,
    required this.location,
  });

  Map<String, dynamic> toJson() {
    return {
      'id': id,
      'status': status,
      'date': date,
      'type': type,
      'description': description,
      'zone': zone,
      'location': location
    };
  }

  factory Report.fromJson(Map<String, dynamic> json) {
    try {
      return Report(
        id: json['id'] as String,
        status: json['status'] as String,
        date: json['date'] as String,
        type: json['type'] as String,
        description: json['description'] as String,
        zone: json['zone'] as String,
        location: json['location'] as String,
      );
    } catch (e) {
      throw const FormatException('Failed to load Report');
    }
  }
}
