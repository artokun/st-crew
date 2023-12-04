import 'package:freezed_annotation/freezed_annotation.dart';
import 'dart:convert';

part 'generated.freezed.dart';
part 'generated.g.dart';

@freezed
class Generated with _$Generated {
    const factory Generated({
        required Command command,
        String? withStuff,
    }) = _Generated;

    factory Generated.fromJson(Map<String, dynamic> json) => _$GeneratedFromJson(json);
}

enum Command {
    GET_SERVER_INFO,
    SOME_OTHER_COMMAND
}

final commandValues = EnumValues({
    "get_server_info": Command.GET_SERVER_INFO,
    "some_other_command": Command.SOME_OTHER_COMMAND
});

class EnumValues<T> {
    Map<String, T> map;
    late Map<T, String> reverseMap;

    EnumValues(this.map);

    Map<T, String> get reverse {
        reverseMap = map.map((k, v) => MapEntry(v, k));
        return reverseMap;
    }
}
