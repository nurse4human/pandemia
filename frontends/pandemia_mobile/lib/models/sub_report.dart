// this code is autogenerated by ansvia-vscode extension.
// please don't edit this by hand
// use 'ansvia-vscode extension > Edit Model fields' instead.
import 'package:equatable/equatable.dart';
import 'package:meta/meta.dart';

/// Model for SubReport
@immutable
class SubReport extends Equatable {
  final int id;
  final int creatorId;
  final String creatorName;
  final String fullName;
  final int age;
  final String residenceAddress;
  final String gender;
  final String comingFrom;
  final String arrivalDate;
  final int healty;
  final String desc;
  final int status;
  final List<String> meta;
  final String ts;

  SubReport(
      this.id,
      this.creatorId,
      this.creatorName,
      this.fullName,
      this.age,
      this.residenceAddress,
      this.gender,
      this.comingFrom,
      this.arrivalDate,
      this.healty,
      this.desc,
      this.status,
      this.meta,
      this.ts)
      : super([
          id,
          creatorId,
          creatorName,
          fullName,
          age,
          residenceAddress,
          gender,
          comingFrom,
          arrivalDate,
          healty,
          desc,
          status,
          meta,
          ts
        ]);

  Map<String, dynamic> toMap() {
    Map<String, dynamic> data = Map();
    data["id"] = this.id;
    data["creator_id"] = this.creatorId;
    data["creator_name"] = this.creatorName;
    data["full_name"] = this.fullName;
    data["age"] = this.age;
    data["residence_address"] = this.residenceAddress;
    data["gender"] = this.gender;
    data["coming_from"] = this.comingFrom;
    data["arrival_date"] = this.arrivalDate;
    data["healty"] = this.healty;
    data["desc"] = this.desc;
    data["status"] = this.status;
    data["meta"] = this.meta;
    data["ts"] = this.ts;
    return data;
  }

  static SubReport fromMap(Map<String, dynamic> data) {
    assert(data['creator_id'] != null, "SubReport.creator_id is null");
    assert(data['creator_name'] != null, "SubReport.creator_name is null");
    assert(data['full_name'] != null, "SubReport.full_name is null");
    assert(data['age'] != null, "SubReport.age is null");
    assert(data['residence_address'] != null,
        "SubReport.residence_address is null");
    assert(data['gender'] != null, "SubReport.gender is null");
    assert(
        data['coming_from'] != null, "SubReport.coming_from is null");
    assert(data['arrival_date'] != null, "SubReport.arrival_date is null");
    assert(data['healty'] != null, "SubReport.healty is null");
    assert(data['desc'] != null, "SubReport.desc is null");
    assert(data['status'] != null, "SubReport.status is null");
    assert(data['ts'] != null, "SubReport.ts is null");
    return SubReport(
        data['id'] as int,
        data['creator_id'] as int,
        data['creator_name'] as String,
        data['full_name'] as String,
        data['age'] as int,
        data['residence_address'] as String,
        data['gender'] as String,
        data['coming_from'] as String,
        data['arrival_date'] as String,
        data['healty'] as int,
        data['desc'] as String,
        data['status'] as int,
        data['meta'] != null ? List.from(data['meta']) : [],
        data['ts'] as String);
  }

  SubReport copy(
      {int creatorId,
      String creatorName,
      String fullName,
      int age,
      String residenceAddress,
      String gender,
      String comingFrom,
      String arrivalDate,
      int healty,
      String desc,
      int status,
      List<String> meta,
      String ts}) {
    return SubReport(
        this.id,
        creatorId ?? this.creatorId,
        creatorName ?? this.creatorName,
        fullName ?? this.fullName,
        age ?? this.age,
        residenceAddress ?? this.residenceAddress,
        gender ?? this.gender,
        comingFrom ?? this.comingFrom,
        arrivalDate ?? this.arrivalDate,
        healty ?? this.healty,
        desc ?? this.desc,
        status ?? this.status,
        meta ?? this.meta,
        ts ?? this.ts);
  }
}
