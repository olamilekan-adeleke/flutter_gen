use crate::lib::utils::capitalize_first_word;

#[derive(Debug)]
pub struct BlocBaseText;

impl BlocBaseText {
    pub fn bloc_state(class_name: &str) -> String {
        let dart_code = format!(
            r#"
            part of '{0}_bloc.dart';

            abstract class {1}State extends Equatable {{
              const {1}State();

              @override
              List<Object> get props => [];
            }}

            class {1}Initial extends {1}State {{

            }}

            class {1}Loading extends {1}State {{

            }}

            class {1}Success extends {1}State {{

              const {1}Success();

              @override
              List<Object> get props => [];
            }}

            class {1}Error extends {1}State {{
              final String message;

              const {1}Error(this.message);

              @override
              List<Object> get props => [message];
            }}
            "#,
            class_name.to_string().to_lowercase(),
            capitalize_first_word(class_name)
        );

        dart_code.to_string()
    }

    pub fn bloc_event(class_name: &str) -> String {
        let dart_code = format!(
            r#"
            part of '{0}_bloc.dart';

            abstract class {1}Event extends Equatable {{
              const {1}Event();

              @override
              List<Object> get props => [];
            }}
            "#,
            class_name.to_string().to_lowercase(),
            capitalize_first_word(class_name)
        );

        dart_code.to_string()
    }

    pub fn bloc_main_class(class_name: &str) -> String {
        let dart_code = format!(
            r#"
            import 'package:equatable/equatable.dart';
            import 'package:flutter_bloc/flutter_bloc.dart';

            part '{0}_event.dart';
            part '{0}_state.dart';

            class {1}Bloc extends Bloc<{1}Event, {1}State> {{
              {1}Bloc() : super({1}Initial()) {{
                on<{1}Event>((event, emit) async {{
                  // TODO: implement event handler
                }});
              }}
            }}
            "#,
            class_name.to_string().to_lowercase(),
            capitalize_first_word(class_name)
        );

        dart_code.to_string()
    }
}

// test
//

mod tests {
    use crate::features::bloc_gen::data::bloc_base_text::BlocBaseText;

    #[test]
    fn test_bloc_state() {
        assert_eq!(
            BlocBaseText::bloc_state("Test").trim(),
            r#"
            part of 'test_bloc.dart';

            abstract class TestState extends Equatable {
              const TestState();

              @override
              List<Object> get props => [];
            }

            class TestInitial extends TestState {

            }

            class TestLoading extends TestState {

            }

            class TestSuccess extends TestState {

              const TestSuccess();

              @override
              List<Object> get props => [];
            }

            class TestError extends TestState {
              final String message;

              const TestError(this.message);

              @override
              List<Object> get props => [message];
            }
            "#
            .trim(),
        );
    }

    #[test]
    fn test_bloc_event() {
        assert_eq!(
            BlocBaseText::bloc_event("Test").trim(),
            r#"
            part of 'test_bloc.dart';

            abstract class TestEvent extends Equatable {
              const TestEvent();

              @override
              List<Object> get props => [];
            }
            "#
            .trim(),
        );
    }

    #[test]
    fn test_bloc_main_class() {
        assert_eq!(
            BlocBaseText::bloc_main_class("Test").trim(),
            r#"
            import 'package:equatable/equatable.dart';
            import 'package:flutter_bloc/flutter_bloc.dart';

            part 'test_event.dart';
            part 'test_state.dart';

            class TestBloc extends Bloc<TestEvent, TestState> {
              TestBloc() : super(TestInitial()) {
                on<TestEvent>((event, emit) async {
                  // TODO: implement event handler
                });
              }
            }
            "#
            .trim(),
        );
    }
}
