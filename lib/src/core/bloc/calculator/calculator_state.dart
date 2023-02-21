import 'package:freezed_annotation/freezed_annotation.dart';

part 'calculator_state.freezed.dart';

@freezed
class CalculatorState with _$CalculatorState {
  const factory CalculatorState.loading() = Initial;
  const factory CalculatorState.targets(String result) = Result;
}
