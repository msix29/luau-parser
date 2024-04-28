local game: DataModel
local function getmetatable(t: {}): {}?
end
local function error(message: any, level: number?): ()
end

type debug = {
	info:
		((thread: thread, level: number, options: string) -> ...any)
		& ((functionOrLevel: () -> () | number, options: string) -> ...any),
	traceback:
		((level: number?) -> string)
		& ((message: string, level: number?) -> string)
		& ((thread: thread, message: string, level: number?) -> string),
	profilebegin: (label: string) -> (),
	profileend: () -> (),
	setmemorycategory: (tag: string) -> (),
	resetmemorycategory: () -> (),
}
local debug: debug

type task = {
	defer: (f: () -> () | thread, ...any) -> any,
	desynchronize: () -> (),
	spawn: (f: () -> () | thread, ...any) -> any,
	synchronize: () -> (),
	delay: (sec: number?, f: () -> () | thread, ...any) -> any,
	wait: (sec: number?) -> number,
	cancel: (thread: thread) -> ()
}
local task: task

local function tonumber(value: any, radix: number?): number
end
local function pcall(f: () -> (), ...: any...): (boolean, ...any)
end

--@deprecated
local function collectgarbage(mode: string): number
end

type utf8 = {
	char: (code: number, ...number) -> string,
	charpattern: string,
	codes: (s: string) -> () -> (),
	codepoint: (s: string, i: number?, j: number?) -> (number, ...number),
	len: (s: string, i: number?, j: number?) -> (number?, number?),
	offset: (s: string, n: number?, i: number?) -> number,
	graphemes: (str: string, i: number?, j: number?) -> () -> (),
	nfcnormalize: (str: string) -> string,
	nfdnormalize: (str: string) -> string,
}
local utf8: utf8

local function getfenv(target: any?): { [string]: any }
end

local plugin: Plugin

local function pairs<T>(t: T): ((T, any?) -> (any, any), T, nil)
end
local function gcinfo(): number
end
local function ypcall(f: () -> (), ...: any...): (boolean, ...any)
end
local function xpcall(f: () -> (), err: () -> (), ...: any...): (boolean, ...any)
end
local workspace: Workspace
local _VERSION: string
type string = {
	pack: (fmt: string, ...any) -> string,
	packsize: (fmt: string) -> number,
	rep: (s: string, n: number) -> string,
	reverse: (s: string) -> string,
	sub: (s: string, i: number, j: number?) -> string,
	unpack: (fmt: string, s: string, pos: number?) -> ...any,
	upper: (s: string) -> string,
	split: (s: string, separator: string?) -> { string },
	byte: (s: string, i: number?, j: number?) -> ...number,
	char: (byte: number, ...number) -> string,
	find: (s: string, pattern: string, init: number?, plain: boolean?) -> (number?, number?),
	format: (s: string, ...any) -> string,
	gmatch: (s: string, pattern: string) -> () -> ...string,
	gsub: (
		s: string,
		pattern: string,
		repl: string | { [string]: string } | ((...string) -> string),
		n: number?
	) -> (string, number),
	len: (s: string) -> number,
	lower: (s: string) -> string,
	match: (s: string, pattern: string, init: number?) -> string?,
}
local string: string
local function warn(...: any...): ()
end
local function UserSettings(): UserSettings
end
local function tostring(value: any): string
end
local function wait(seconds: number?): (number, number)
end
local function unpack<T>(list: {T}, i: number?, j: number?): ...T
end
local function tick(): number
end
local function newproxy(addmt: boolean): any
end
local function time(): number
end
local function elapsedTime(): number
end
type table = {
	maxn: (t: {}) -> number,
	find: (t: {}, value: any, init: number?) -> number?,
	foreach: (t: {}, f: (any, any) -> ()) -> (),
	foreachi: (t: {}, f: (number, any) -> ()) -> (),
	getn: (t: {}) -> number,
	concat: (t: {}, sep: string?, i: number?, j: number?) -> string,
	insert:
		((t: {}, value: any) -> ())
		& ((t: {}, pos: number, value: any) -> ()),
	move: (a1: {}, f: number, e: number, t: number, a2: {}?) -> (),
	pack: (...any) -> { n: number, [number]: any },
	remove: (t: {}, pos: number?) -> any?,
	sort: (t: {}, comp: ((any, any) -> boolean)?) -> (),
	unpack: (t: {}, i: number?, j: number?) -> ...any,
	clear: (t: {}) -> (),
	clone: (t: {}) -> {},
	create: (count: number, value: any?) -> {},
	freeze: (t: {}) -> {},
	isfrozen: (t: {}) -> boolean
}
local table: table
local function setfenv(target: () -> () | number, env: { [string]: any }): () -> ()?
end
type os = {
	clock: () -> number,
	date: (format: string?, time: number?) -> string | DateTypeResult,
	difftime: (t2: number | DateTypeResult, t1: number | DateTypeResult) -> number,
	time: (t: DateTypeArg?) -> number,
}
local os: os

local function type(value: any): string
end
local function ipairs<T>(t: T): ((T, number) -> (number, any), T, number)
end
type coroutine = {
	create: (f: () -> ()) -> thread,
	isyieldable: () -> boolean,
	resume: (co: thread, ...any) -> (boolean, ...any),
	running: () -> thread,
	status: (co: thread) -> string,
	wrap: (f: () -> ()) -> () -> (),
	yield: (...any) -> ...any,
	close: (thread: thread) -> (boolean, ...any),
}
local coroutine: coroutine
local shared: any
local function typeof(value: any): string
end
local function spawn(callback: () -> ()): ()
end

local _G: {}

local function settings(): GlobalSettings
end
-- local function require(module: any): any
-- end
type bit32 = {
	lshift: (x: number, disp: number) -> number,
	rrotate: (x: number, disp: number) -> number,
	rshift: (x: number, disp: number) -> number,
	arshift: (x: number, disp: number) -> number,
	band: (...number) -> number,
	bnot: (x: number) -> number,
	bor: (...number) -> number,
	btest: (x: number, ...number) -> boolean,
	bxor: (...number) -> number,
	extract: (n: number, field: number, width: number?) -> number,
	replace: (n: number, v: number, field: number, width: number?) -> number,
	lrotate: (x: number, disp: number) -> number,
}
local bit32: bit32
local function select(index: string | number, ...: any...): any
end
local function setmetatable(t: {}, metatable: {} | nil): {}
end
local function rawset(t: {}, index: any, value: any): {}
end
local function rawget(t: {}, index: any): any
end
local function rawlen(obj: string | {}): number
end
local function rawequal(v1: any, v2: any): boolean
end
local function print(...: any...): ()
end

type math = {
	exp: (x: number) -> number,
	floor: (x: number) -> number,
	fmod: (x: number, y: number) -> number,
	frexp: (x: number) -> (number, number),
	huge: number,
	ldexp: (m: number, e: number) -> number,
	log: (x: number, base: number?) -> number,
	log10: (x: number) -> number,
	max: (x: number, ...any) -> number,
	min: (x: number, ...any) -> number,
	abs: (x: number) -> number,
	modf: (x: number) -> (number, number),
	pi: number,
	pow: (x: number, y: number) -> number,
	rad: (x: number) -> number,
	random: (m: number?, n: number?) -> number,
	randomseed: (x: number) -> (),
	sin: (x: number) -> number,
	sinh: (x: number) -> number,
	sqrt: (x: number) -> number,
	tan: (x: number) -> number,
	acos: (x: number) -> number,
	tanh: (x: number) -> number,
	clamp: (n: number, min: number, max: number) -> number,
	noise: (x: number, y: number, z: number) -> number,
	sign: (x: number) -> number,
	round: (x: number) -> number,
	asin: (x: number) -> number,
	atan: (y: number) -> number,
	atan2: (y: number, x: number) -> number,
	ceil: (x: number) -> number,
	cos: (x: number) -> number,
	cosh: (x: number) -> number,
	deg: (x: number) -> number,
}
local math: math

local function next<K, V>(t: {[K]: V}, lastKey: K?): (K, V)
end
local function loadstring(src: string, chunkname: string?): (() -> ()?, string?)
end

--@deprecated
local function delay(delayTime: number?, callback: () -> ()): ()
end

local script: LuaSourceContainer

local function assert(value: any, errorMessage: string?): any
end

export type DateTypeArg = {
	day: number?,
	hour: number?,
	isdst: boolean?,
	min: number?,
	month: number?,
	sec: number?,
	wday: number?,
	yday: number?,
	year: number?,
}

export type DateTypeResult = {
	day: number,
	hour: number,
	isdst: boolean,
	min: number,
	month: number,
	sec: number,
	wday: number,
	yday: number,
	year: number,
}
