"use strict"
import { Canva } from "/lib_animation/Mano/Canva.js"

const [_D, _W, c] = [document, window, console]

/**
 * type - {}
 * description: it's an object that holds little functions(methods) for type and instance checkings with type conversions
 */
 
 const type = {
   num: function(n) {
    return typeof n === 'number'
   },
   str: function(s) {
    return typeof s === 'string'
   },
   nill: function(n) {
     return typeof n === 'null'
   },
   unknown: function (u) {
      return typeof u === 'undefined'
   },
   arr: function(a) {
     return Array.isArray(a)
   },
   obj: function(o) {
      return o instanceof Object && !Array.isArray(o)
   },
   func: function(f) {
      return f instanceof Function
   },
   regex: function(x) {
      return x instanceof RegExp
   },
  /* toRGB,
   toHEX,
   toHSL,
   toTurn,
   toDeg,
   toRad,
   */
   isInstance: function(o1,o2) {
      return o1 instanceof o2
   },
   isNegative: function(n) {
     return n < 0
   }
 }

/**
 * utility functions and equations made for the mano object
 */

 function dq(elem) {
   return _D.querySelector(elem)
 }
 
 function dqA(nodelist) {
   return _D.querySelectorAll(nodelist)
 }
 
 function dc(n) {
  return _D.createElement(n) 
 }
 
 function inherit(obj){
   if(obj === null) throw TypeError()
   
   if(Object.create) return Object.create(obj)
   
   function o() {}
   o.prototype = obj
   return new o()
 }
 
 function extend(from, to) {
   for(let prop in from) to[prop] = from[prop]
   return to
 }
 
 /**
  * createSubclass - (extends) in ES6+
  * creates a subset of a super class so as to inherit it's methods and ppts
  */
 function createSubclass(superclass, subclass) {
   subclass.prototype = inherit(superclass.prototype)
   subclass.prototype.constructor = subclass
   
   return subclass
 }
 
 function toNode() {
   
 }
 
 /**
 * mano.js - factory approach 
 *  @param { type: Object } name: data
 * 
 * description: an animation Object using the old prototype (without class keyword )
 * which makes it a kinda factory function 
 * Return: an object ....
 */
 
 function mano(data = {}) {
   _update()
   const m = inherit(mano.methods)
   
   m.select = dqA(data?.select)
   m.version = "mano@1.0.1"
  
   return m
 }
 
 /**
  * mano.methods - {}
  * it's an object that holds all the essential methods of mano , making use of (inherit) function 
  */
  mano.methods = {
    constructor: mano,
    
    on: function(events = {}) {
      
    },
    
    css: function(styles = {}) {
      
    },
    
  }
 
export { mano, dq, dqA, dc, inherit, extend, createSubclass, type}

/**
 * look at the js animation API
 * new Animation ()
 * Element.animate
 */

function _update() { return c.warn("mano.js is currently under development\n\vby nonso01 and friends \n 01-01-23 to ... when it's ready ") }
