package hxrust.native;

#if cpp
@:include("hxrust.h")
@:native("HxRustNative")
extern class RustNative
{
    @:native("hxrust_api_version")
    public static function apiVersion():Int;

    @:native("hxrust_is_available")
    public static function isAvailable():Bool;

    @:native("hxrust_add")
    public static function add(a:Float, b:Float):Float;

    @:native("hxrust_subtract")
    public static function subtract(a:Float, b:Float):Float;

    @:native("hxrust_multiply")
    public static function multiply(a:Float, b:Float):Float;

    @:native("hxrust_divide")
    public static function divide(a:Float, b:Float):Float;

    @:native("hxrust_power")
    public static function power(base:Float, exponent:Float):Float;

    @:native("hxrust_sqrt")
    public static function sqrt(value:Float):Float;

    @:native("hxrust_abs")
    public static function abs(value:Float):Float;

    @:native("hxrust_min")
    public static function min(a:Float, b:Float):Float;

    @:native("hxrust_max")
    public static function max(a:Float, b:Float):Float;

    @:native("hxrust_clamp")
    public static function clamp(
        value:Float,
        minimum:Float,
        maximum:Float
    ):Float;

    @:native("hxrust_lerp")
    public static function lerp(
        start:Float,
        end:Float,
        ratio:Float
    ):Float;
}
#else
class RustNative
{
    public static function apiVersion():Int
    {
        return 0;
    }

    public static function isAvailable():Bool
    {
        return false;
    }

    static function unsupported():Dynamic
    {
        throw "hxrust requires a native C++ target.";
    }

    public static function add(a:Float, b:Float):Float
    {
        return unsupported();
    }

    public static function subtract(a:Float, b:Float):Float
    {
        return unsupported();
    }

    public static function multiply(a:Float, b:Float):Float
    {
        return unsupported();
    }

    public static function divide(a:Float, b:Float):Float
    {
        return unsupported();
    }

    public static function power(a:Float, b:Float):Float
    {
        return unsupported();
    }

    public static function sqrt(value:Float):Float
    {
        return unsupported();
    }

    public static function abs(value:Float):Float
    {
        return unsupported();
    }

    public static function min(a:Float, b:Float):Float
    {
        return unsupported();
    }

    public static function max(a:Float, b:Float):Float
    {
        return unsupported();
    }

    public static function clamp(
        value:Float,
        minimum:Float,
        maximum:Float
    ):Float
    {
        return unsupported();
    }

    public static function lerp(
        start:Float,
        end:Float,
        ratio:Float
    ):Float
    {
        return unsupported();
    }
}
#end
