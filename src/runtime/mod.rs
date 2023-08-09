pub mod error;
pub mod executor;
pub mod frame;

#[cfg(test)]
mod test {
    use crate::compiler::test::get_test_program;
    use crate::intermediate::constant::Constant;
    use crate::runtime::executor::evalutate_expression;
    use crate::runtime::frame::Frame;
    use rstest::*;

    #[test]
    pub fn test_simple_calculator() {
        let program = get_test_program("valid", "simple_calculator.lrlang");
        let frame = Frame::default();
        let (_, frame) = evalutate_expression(frame, &program).unwrap();

        assert_eq!(
            frame.variable_value("my_variable").unwrap(),
            Constant::Int(5)
        );
        assert_eq!(
            frame.variable_value("my_variable_1").unwrap(),
            Constant::Int(5)
        );
        assert_eq!(frame.variable_value("sum").unwrap(), Constant::Int(10));
        assert_eq!(frame.variable_value("mult").unwrap(), Constant::Int(1200));
        assert_eq!(frame.variable_value("div").unwrap(), Constant::Int(60));
        assert_eq!(
            frame.variable_value("expression").unwrap(),
            Constant::Int(125)
        );
        assert_eq!(
            frame.variable_value("float").unwrap(),
            Constant::Float(5.5)
        );
        assert_eq!(
            frame.variable_value("float_plus_float").unwrap(),
            Constant::Float(11.0)
        );
        assert_eq!(
            frame.variable_value("float_plus_int").unwrap(),
            Constant::Float(10.5)
        );
        assert_eq!(
            frame.variable_value("int_plus_float").unwrap(),
            Constant::Float(10.5)
        );
        assert_eq!(
            frame.variable_value("float_minus_float").unwrap(),
            Constant::Float(0.5)
        );
        assert_eq!(
            frame.variable_value("float_minus_int").unwrap(),
            Constant::Float(0.5)
        );
        assert_eq!(
            frame.variable_value("int_minus_float").unwrap(),
            Constant::Float(-0.5)
        );
        assert_eq!(
            frame.variable_value("float_times_float").unwrap(),
            Constant::Float(11.0)
        );
        assert_eq!(
            frame.variable_value("float_times_int").unwrap(),
            Constant::Float(11.0)
        );
        assert_eq!(
            frame.variable_value("int_times_float").unwrap(),
            Constant::Float(11.0)
        );
        assert_eq!(
            frame.variable_value("float_div_float").unwrap(),
            Constant::Float(1.1)
        );
        assert_eq!(
            frame.variable_value("float_div_int").unwrap(),
            Constant::Float(1.1)
        );
        assert_eq!(
            frame.variable_value("int_div_float").unwrap(),
            Constant::Float(1.1)
        );
    }

    #[test]
    pub fn test_string_operations() {
        let program = get_test_program("valid", "string_operations.lrlang");
        let frame = Frame::default();
        let (_, frame) = evalutate_expression(frame, &program).unwrap();

        assert_eq!(
            frame.variable_value("string").unwrap(),
            Constant::String("100".to_owned())
        );
        assert_eq!(
            frame.variable_value("string_concat_1").unwrap(),
            Constant::String("abc100".to_owned())
        );
        assert_eq!(
            frame.variable_value("string_concat_2").unwrap(),
            Constant::String("100100".to_owned())
        );
        assert_eq!(
            frame.variable_value("string_concat_3").unwrap(),
            Constant::String("100 abc".to_owned())
        );
        assert_eq!(
            frame.variable_value("string_concat_4").unwrap(),
            Constant::String("abcefg".to_owned())
        );
        assert_eq!(
            frame.variable_value("string_plus_int").unwrap(),
            Constant::String("int=5".to_owned())
        );
        assert_eq!(
            frame.variable_value("string_plus_float").unwrap(),
            Constant::String("float=5.5".to_owned())
        );
        assert_eq!(
            frame.variable_value("string_plus_int_var").unwrap(),
            Constant::String("int=5".to_owned())
        );
        assert_eq!(
            frame.variable_value("string_plus_float_var").unwrap(),
            Constant::String("float=5.5".to_owned())
        );
        assert_eq!(
            frame.variable_value("redefine").unwrap(),
            Constant::String("new_value".to_owned())
        );
    }
    #[test]
    pub fn test_circle_square() {
        let program = get_test_program("valid", "circle_square.lrlang");
        let frame = Frame::default();
        let (_, frame) = evalutate_expression(frame, &program).unwrap();

        assert_eq!(
            frame.variable_value("hello_world").unwrap(),
            Constant::String("Hello World".to_owned())
        );
        assert_eq!(
            frame.variable_value("value").unwrap(),
            Constant::String(
                "The square of the circle with the r = 5 is 157. It is > 100. It is <= 200."
                    .to_owned()
            )
        );
    }

    #[rstest]
    #[case(
        "string_mult_num",
        "Unable to evalutate expression (string * num): Operation String * Int is not defined"
    )]
    #[case(
        "string_div_num",
        "Unable to evalutate expression (\"100\" / 5): Operation String / Int is not defined"
    )]
    #[case(
        "string_minus_num",
        "Unable to evalutate expression (string - num): Operation String - Int is not defined"
    )]
    #[case(
        "float_minus_string",
        "Unable to evalutate expression (num - string): Operation Float - String is not defined"
    )]
    #[case(
        "assign_int_to_string",
        "Unable to assign the value of type Int to variable 'string' of type String"
    )]
    pub fn test_runtime_error(#[case] file: &str, #[case] expected_error: &str) {
        let program = get_test_program("runtime_error", &format!("{}.lrlang", file));
        let frame = Frame::default();
        let result = evalutate_expression(frame, &program);
        assert!(result.is_err());
        let error = result.err().unwrap().to_string();
        assert_eq!(expected_error, error);
    }

    #[test]
    pub fn test_simple_blocks() {
        let program = get_test_program("valid", "simple_blocks.lrlang");
        let frame = Frame::default();
        let (_, frame) = evalutate_expression(frame, &program).unwrap();

        assert_eq!(frame.variable_value("var_1").unwrap(), Constant::Int(7));
        assert_eq!(frame.variable_value("result").unwrap(), Constant::Int(13));
        assert!(frame.variable_value("var_2").is_err())
    }

    #[test]
    pub fn test_if_blocks() {
        let program = get_test_program("valid", "if_blocks.lrlang");
        let frame = Frame::default();
        let (_, frame) = evalutate_expression(frame, &program).unwrap();

        assert_eq!(frame.variable_value("var_1").unwrap(), Constant::Int(7));
        assert!(frame.variable_value("var_2").is_err());
    }
}
