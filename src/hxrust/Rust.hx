package hxrust;

import hxrust.native.RustNative;
import haxe.ds.StringMap;
import StringTools;

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
        return VERSION;
    }

    public static function apiVersion():Int
    {
        return available ? RustNative.apiVersion() : 0;
    }

    public static function platform():String
    {
        return Sys.systemName();
    }

    public static function architecture():String
    {
        #if (cpp && HXCPP_ARM64)
        return "aarch64";
        #elseif (cpp && HXCPP_ARM)
        return "arm";
        #elseif (cpp && HXCPP_M64)
        return "x86_64";
        #elseif cpp
        return "x86";
        #else
        return "unknown";
        #end
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

    public static function modulo(a:Float, b:Float):Float
    {
        return a % b;
    }

    public static function floor(value:Float):Int
    {
        return Math.floor(value);
    }

    public static function ceil(value:Float):Int
    {
        return Math.ceil(value);
    }

    public static function round(value:Float):Int
    {
        return Math.round(value);
    }

    public static function sin(value:Float):Float
    {
        return Math.sin(value);
    }

    public static function cos(value:Float):Float
    {
        return Math.cos(value);
    }

    public static function tan(value:Float):Float
    {
        return Math.tan(value);
    }

    public static function log(value:Float):Float
    {
        return Math.log(value);
    }

    public static function exp(value:Float):Float
    {
        return Math.exp(value);
    }

    public static function random():Float
    {
        return Math.random();
    }

    public static function randomInt(minimum:Int, maximum:Int):Int
    {
        if (minimum > maximum)
        {
            throw "Minimum cannot be greater than maximum.";
        }

        return minimum + Std.random(maximum - minimum + 1);
    }

    public static function isNaN(value:Float):Bool
    {
        return Math.isNaN(value);
    }

    public static function isFinite(value:Float):Bool
    {
        return Math.isFinite(value);
    }

    public static function isInfinite(value:Float):Bool
    {
        return !Math.isNaN(value) && !Math.isFinite(value);
    }

    public static function sign(value:Float):Int
    {
        if (Math.isNaN(value))
        {
            return 0;
        }

        if (value > 0)
        {
            return 1;
        }

        if (value < 0)
        {
            return -1;
        }

        return 0;
    }

    public static function distance(
        x1:Float,
        y1:Float,
        x2:Float,
        y2:Float
    ):Float
    {
        var dx = x2 - x1;
        var dy = y2 - y1;

        return Math.sqrt(dx * dx + dy * dy);
    }

    public static function stringLength(value:String):Int
    {
        return value == null ? 0 : value.length;
    }

    public static function stringEquals(
        first:String,
        second:String
    ):Bool
    {
        return first == second;
    }

    public static function stringConcat(
        first:String,
        second:String
    ):String
    {
        return (first == null ? "" : first)
            + (second == null ? "" : second);
    }

    public static function stringUpper(value:String):String
    {
        return value == null ? "" : value.toUpperCase();
    }

    public static function stringLower(value:String):String
    {
        return value == null ? "" : value.toLowerCase();
    }

    public static function stringTrim(value:String):String
    {
        return value == null ? "" : StringTools.trim(value);
    }

    public static function stringContains(
        value:String,
        search:String
    ):Bool
    {
        if (value == null || search == null)
        {
            return false;
        }

        return value.indexOf(search) != -1;
    }

    public static function stringStartsWith(
        value:String,
        prefix:String
    ):Bool
    {
        return value != null
            && prefix != null
            && StringTools.startsWith(value, prefix);
    }

    public static function stringEndsWith(
        value:String,
        suffix:String
    ):Bool
    {
        return value != null
            && suffix != null
            && StringTools.endsWith(value, suffix);
    }

    public static function stringReplace(
        value:String,
        search:String,
        replacement:String
    ):String
    {
        if (value == null)
        {
            return "";
        }

        if (search == null || search.length == 0)
        {
            return value;
        }

        return StringTools.replace(
            value,
            search,
            replacement == null ? "" : replacement
        );
    }

    public static function stringRepeat(
        value:String,
        count:Int
    ):String
    {
        if (value == null || count <= 0)
        {
            return "";
        }

        var buffer = new StringBuf();

        for (i in 0...count)
        {
            buffer.add(value);
        }

        return buffer.toString();
    }

    public static function stringReverse(value:String):String
    {
        if (value == null)
        {
            return "";
        }

        var result = new StringBuf();

        for (i in 0...value.length)
        {
            result.add(value.charAt(value.length - i - 1));
        }

        return result.toString();
    }

    public static function stringSplit(
        value:String,
        delimiter:String
    ):Array<String>
    {
        if (value == null)
        {
            return [];
        }

        return value.split(delimiter);
    }

    public static function parseInt(value:String):Null<Int>
    {
        return Std.parseInt(value);
    }

    public static function parseFloat(value:String):Float
    {
        return Std.parseFloat(value);
    }

    public static function toString(value:Dynamic):String
    {
        return Std.string(value);
    }

    public static function toInt(value:Float):Int
    {
        return Std.int(value);
    }

    public static function arraySum(values:Array<Float>):Float
    {
        if (values == null)
        {
            return 0;
        }

        var result:Float = 0;

        for (value in values)
        {
            result += value;
        }

        return result;
    }

    public static function arrayAverage(
        values:Array<Float>
    ):Float
    {
        if (values == null || values.length == 0)
        {
            return Math.NaN;
        }

        return arraySum(values) / values.length;
    }

    public static function arrayMin(
        values:Array<Float>
    ):Float
    {
        if (values == null || values.length == 0)
        {
            return Math.NaN;
        }

        var result = values[0];

        for (value in values)
        {
            result = Math.min(result, value);
        }

        return result;
    }

    public static function arrayMax(
        values:Array<Float>
    ):Float
    {
        if (values == null || values.length == 0)
        {
            return Math.NaN;
        }

        var result = values[0];

        for (value in values)
        {
            result = Math.max(result, value);
        }

        return result;
    }

    public static function arrayReverse<T>(
        values:Array<T>
    ):Array<T>
    {
        if (values == null)
        {
            return [];
        }

        var result = values.copy();
        result.reverse();

        return result;
    }

    public static function arrayContains<T>(
        values:Array<T>,
        value:T
    ):Bool
    {
        return values != null && values.indexOf(value) != -1;
    }

    public static function arrayUnique<T>(
        values:Array<T>
    ):Array<T>
    {
        var result:Array<T> = [];

        if (values == null)
        {
            return result;
        }

        for (value in values)
        {
            if (result.indexOf(value) == -1)
            {
                result.push(value);
            }
        }

        return result;
    }

    public static function arrayShuffle<T>(
        values:Array<T>
    ):Array<T>
    {
        if (values == null)
        {
            return [];
        }

        var result = values.copy();

        var i = result.length;

        while (i > 1)
        {
            i--;

            var j = Std.random(i + 1);
            var temp = result[i];

            result[i] = result[j];
            result[j] = temp;
        }

        return result;
    }

    public static function createMap():StringMap<String>
    {
        return new StringMap<String>();
    }

    public static function timestamp():Float
    {
        return Date.now().getTime();
    }

    public static function sleep(seconds:Float):Void
    {
        #if sys
        Sys.sleep(seconds < 0 ? 0 : seconds);
        #else
        throw "Sleep is not supported on this target.";
        #end
    }

    public static function assert(
        condition:Bool,
        ?message:String
    ):Void
    {
        if (!condition)
        {
            throw message == null
                ? "hxrust assertion failed."
                : message;
        }
    }
}
