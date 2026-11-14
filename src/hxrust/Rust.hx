package hxrust;

class Rust
{
    public static inline var VERSION:String = "0.1.0";
    public static inline var API_VERSION:Int = 1;

    public static var available(get, never):Bool;

    static function get_available():Bool
    {
        return RustNative.isAvailable();
    }

    public static function version():String
    {
        return RustNative.version();
    }

    public static function apiVersion():Int
    {
        return RustNative.apiVersion();
    }

    public static function platform():String
    {
        return RustNative.platform();
    }

    public static function architecture():String
    {
        return RustNative.architecture();
    }

    public static function add(a:Float, b:Float):Float
    {
        return RustNative.add(a, b);
    }

    public static function subtract(a:Float, b:Float):Float
    {
        return RustNative.subtract(a, b);
    }

    public static function multiply(a:Float, b:Float):Float
    {
        return RustNative.multiply(a, b);
    }

    public static function divide(a:Float, b:Float):Float
    {
        return RustNative.divide(a, b);
    }

    public static function power(base:Float, exponent:Float):Float
    {
        return RustNative.power(base, exponent);
    }

    public static function sqrt(value:Float):Float
    {
        return RustNative.sqrt(value);
    }

    public static function abs(value:Float):Float
    {
        return RustNative.abs(value);
    }

    public static function min(a:Float, b:Float):Float
    {
        return RustNative.min(a, b);
    }

    public static function max(a:Float, b:Float):Float
    {
        return RustNative.max(a, b);
    }

    public static function clamp(
        value:Float,
        minimum:Float,
        maximum:Float
    ):Float
    {
        return RustNative.clamp(value, minimum, maximum);
    }

    public static function lerp(
        start:Float,
        end:Float,
        ratio:Float
    ):Float
    {
        return RustNative.lerp(start, end, ratio);
    }

    public static function stringLength(value:String):Int
    {
        return RustNative.stringLength(value);
    }

    public static function stringEquals(
        first:String,
        second:String
    ):Bool
    {
        return RustNative.stringEquals(first, second);
    }

    public static function stringConcat(
        first:String,
        second:String
    ):String
    {
        return RustNative.stringConcat(first, second);
    }
}
