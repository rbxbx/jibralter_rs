/* ;;;;;;;; start Jibralter specs ;;;;;;; */
/* ;;;;;;;; start Jibralter specs ;;;;;;; */

/* Jibralter = require('../jibralter').jibralter */
/* Env = require('../jibralter').env */

/* describe "Jibralter", -> */

/*   it 'tokenizes', -> */
/*     tokens = Jibralter.tokenize("(+ 1 2)") */
/*     expect(tokens).toEqual([ '(', '+', '1', '2', ')' ]) */

/*   it 'parses', -> */
/*     ast = Jibralter.parse("(+ 1 2)") */
/*     expect(ast).toEqual([ '+', 1, 2]) */

/*     ast = Jibralter.parse('1') */
/*     expect(ast).toEqual(1) */

/*   it 'parses nested expressions', -> */
/*     ast = Jibralter.parse("(1 (2 3))") */
/*     expect(ast).toEqual([1, [2, 3]]) */

/*     ast = Jibralter.parse("((1 2))") */
/*     expect(ast).toEqual([[1, 2]]) */

/*     ast = Jibralter.parse("(1 (2 3) 4)") */
/*     expect(ast).toEqual([ 1, [ 2, 3 ], 4 ]) */

/*     ast = Jibralter.parse("(+ 1 (+ 2 3))") */
/*     expect(ast).toEqual([ '+', 1, [ '+', 2, 3 ] ]) */

/*     ast = Jibralter.parse("((1 2) 3)") */
/*     expect(ast).toEqual([ [ 1, 2], 3 ]) */

/*     ast = Jibralter.parse("(()()())") */
/*     expect(ast).toEqual([ [], [], [], ]) */

/*   describe "#evaluate", -> */
/*     evaluate = Jibralter.evaluate */

/*     it "variables", -> */
/*       ast = [ "foo" ] */
/*       env = new Env */
/*       env.set("foo", 1) */

/*       evaluation = evaluate(ast, env) */
/*       expect(evaluation).toEqual(1) */

/*     it "constant literal", -> */
/*       ast = [ 1 ] */

/*       evaluation = evaluate(ast) */
/*       expect(evaluation).toEqual(1) */

/*     it "quotes", -> */
/*       ast = [ "quote", [1]] */
/*       evaluation = Jibralter.evaluate(ast) */
/*       expect(evaluation).toEqual([1]) */

/* ;;;;;;;; start env specs ;;;;;;; */
/* ;;;;;;;; start env specs ;;;;;;; */

/* Env = require('../jibralter').env */

/* describe "Environment", -> */
/*   env = new Env */
/*   variables = { foo: 1, bar: 2 } */

/*   describe "#update", -> */

/*     it "puts variables into the store", -> */
/*       env.update(variables) */
/*       expect(env.variables).toEqual(variables) */

/*     it "updates existing variables with new values", -> */
/*       newValues = { foo: 3, bar: 4 } */
/*       env.update(variables) */
/*       env.update(newValues) */

/*       expect(env.variables).toEqual(newValues) */

/*   describe "#set", -> */
/*     it "sets a single variable in the store", -> */
/*       env.set("foo", 4) */
/*       expect(env.variables.foo).toEqual(4) */

/*   describe "#find", -> */
/*     env.set("foo", 4) */

/*     it "returns the environment containing a variable", -> */
/*       expect(env.find("foo")).toEqual(env) */

/*     it "looks up parent scopes until the variable is found", -> */
/*       childEnv = new Env({}, {}, env) */
/*       expect(childEnv.find("foo")).toEqual(env) */

/*     it "shadows variables", -> */
/*       childEnv = new Env(["foo", "bar", "baz"], [6, 7, 8], env) */

/*       expect(childEnv.find("foo").variables.foo).toEqual(6) */
/*       expect(childEnv.find("bar").variables.bar).toEqual(7) */
/*       expect(childEnv.find("baz").variables.baz).toEqual(8) */
/* } */

/* ;;;;;;;; start Jibralter impl ;;;;;;; */
/* ;;;;;;;; start Jibralter impl ;;;;;;; */

/* _ = require("underscore") */

/* jibralter = {} */

/* class Env */
/*   constructor: (params, args, outer = null) -> */
/*     @variables = {} */
/*     @outer = outer */

/*     unless _.isEmpty(params) or _.isEmpty(args) */

/*       vars = {} */
/*       for [x, y] in _.zip(params, args) */
/*         vars[x] = y */

/*       @update(vars) */

/*   find: (variable) -> */
/*     if _.has(@variables, variable) then this else @outer.find(variable) */

/*   update: (params) -> */
/*     _.extend(@variables, params) */

/*   set: (variable, value) -> */
/*     @variables[variable] = value */

/*   get: (variable) -> @variables[variable] */

/* jibralter.addGlobals = (env) -> */
/*   operators = */
/*     '+': (x, y) -> x + y */
/*     '-': (x, y) -> x - y */
/*     '*': (x, y) -> x * y */
/*     '/': (x, y) -> x / y */
/*     '>': (x, y) -> x > y */
/*     '<': (x, y) -> x < y */
/*     '=': (x, y) -> x == y */
/*     '>=': (x, y) -> x >= y */
/*     '<=': (x, y) -> x <= y */
/*   env.update _.extend(operators, { 'True': true, 'False': false }) */

/* jibralter.globalEnv = do -> */
/*   env = new Env */
/*   jibralter.addGlobals(env) */
/*   env */

/* jibralter.evaluate = (x, env = jibralter.globalEnv) => */
/*   [head, tail] = x */

/*   if _.isString(x) */
/*     env.find(x).get(x) */
/*   else if not _.isArray(x) */
/*     x */
/*   else */
/*     switch head */
/*       when 'quote', 'q' */
/*         tail */
/*       when 'atom?' */
/*         not _.isArray(jibralter.evaluate(tail, env)) */
/*       when 'eq?' */
/*         [exp1, exp2] = tail */
/*         val1 = jibralter.evaluate(exp1, env) */
/*         val2 = jibralter.evaluate(exp2, env) */
/*         (not _.isArray(val1)) and (val1 == val2) */
/*       when 'car' */
/*         _.first jibralter.evaluate(tail, env) */
/*       when 'cdr' */
/*         _.rest jibralter.evaluate(tail, env) */
/*       when 'cons' */
/*         _.map(tail, jibralter.evaluate) */
/*       when 'cond' */
/*         _.each tail, (predicate, exp) -> */
/*           if jibralter.evaluate(predicate, env) then jibralter.evaluate(exp, env) */
/*       when 'null?' */
/*         _.isEmpty(jibralter.evaluate(tail, env)) */
/*       when 'if' */
/*         [test, conseq, alt] = tail */
/*         if jibralter.evaluate(test, env) */
/*           jibralter.evaluate(conseq, env) */
/*         else */
/*           jibralter.evaluate(alt, env) */
/*       when 'set!' */
/*         [variable, exp] = tail */
/*         env.find(variable).set variable, jibralter.evaluate(exp, env) */
/*       when 'define' */
/*         [variable, exp] = tail */
/*         env.set variable, jibralter.evaluate(exp) */
/*       when 'lambda' */
/*         [variables, exp] = tail */
/*         (args) -> jibralter.evaluate(exp, (new Env(variables, args, env))) */
/*       when 'begin' */
/*         for exp in tail */
/*           val = jibralter.evaluate(exp, env) */
/*         val */
/*       else */
/*         [exp, args...] = jibralter.evaluate(exp, env) for exp in x */
/*         exp(args) */

/* jibralter.parse = (string) => jibralter.readFrom(jibralter.tokenize(string)) */

/* jibralter.tokenize = (string) -> */
/*   _.compact string.replace(/\(/gm, " ( ").replace(/\)/gm, " ) ").split(" ") */

/* jibralter.readFrom = (tokens) => */
/*   throw "Syntax Error: unexpected EOF while reading" if _.isEmpty(tokens) */
/*   token = tokens.shift() */
/*   if '(' == token */
/*     list = [] */
/*     while _.first(tokens) != ')' */
/*       list.push(jibralter.readFrom(tokens)) */
/*     tokens.shift() */
/*     return list */
/*   else if ')' == token */
/*     throw "Syntax Error: unexpected ')'" */
/*   else jibralter.atom(token) */

/* jibralter.atom = (token) => */
/*   if _.isFinite(+token) */
/*     +token */
/*   else */
/*     jibralter.toString(token) */

/* jibralter.toString = (exp) => */
/*   unless _.isArray(exp) */
/*     exp.toString() */
/*   else */
/*     "(#{ _.join(_.map(exp, jibralter.toString), ' ') })" */

/* jibralter.repl = (prompt = 'jibralter > ') => */
/*   readline = require("readline") */
/*   rl = readline.createInterface(process.stdin, process.stdout) */
/*   try */
/*     rl.on 'line', (line) -> */
/*       val = jibralter.evaluate(jibralter.parse(line)) */
/*       console.log(jibralter.parse(line)) */
/*       console.log(val) unless _.isEmpty(val) */
/*       rl.prompt() */

/*     rl.on 'close', -> */
/*       console.log("\n Exiting") */
/*       process.exit(0) */

/*     rl.setPrompt(prompt, prompt.length) */
/*     rl.prompt() */
/*   catch err */
/*     console.log "An error occurred: #{err}" */

/* module.exports = { */
/*   env: Env */
/*   jibralter: jibralter */
/* } */

pub mod tok;
