  'use strict'
  const [_W,_D]=[window,document]

  const c=console
  const dq=x=>_D.querySelector(x)
  const dqA=x=>_D.querySelectorAll(x)
  const dc=x=>_D.createElement(x)
  
  const html= dq("html")
  const body= dq("body")
  
  class Anime{
    constructor(data={}){
      this.element
      this.nodeList
      
      _checkElementWithinData.call(this,data)
   
    this._scroll
    // scroll obj key/value
    this._reveal
    // reveal obj key/value
    this._drop
    // drop obj key/value
    this.drag
    // drag obj ket/value
    this.start=false 
    // when initialized
      
    // Element data
    Anime.ATTRIBUTES= this.element.attributes
    Anime.PARENTNODE=this.element.parentNode
    
    Anime.KEYS= Object.keys(data)
    
    Anime.RECT= this.element.getBoundingClientRect()
    Anime.BOXGEOMETRY={
      w: Anime.RECT.width,
      h: Anime.RECT.height,
      x: Anime.RECT.x,
      y: Anime.RECT.y,
      t: Anime.RECT.top,
      b: Anime.RECT.bottom
    }
    
    Anime.TYPE={
      NAME: data.type.name || "reveal",
      DELAY: data.type.delay || 0,
      LOOP: data.type.loop || false,
      RANDOM: Math.floor(Math.random()*data.type.random) || 0,
      SPEED: data.type.speed || .05
    }
    
    Anime.TWO_D=false
    
    _warningAboutChanges()
  }
    
    
    on(event="animationend",f=function(){}){ 
      let arg= arguments
      
      if(arg.length<2 || arg.length>2){
        throw new Error(`function expects two args ,"event_type and callback" but had ${arg.length} arg(s)`)
      }
      
      if((typeof arg[0]!=="string") && (typeof arg[1]!=="function")){
        throw new TypeError
      }
      
      
      const _ev=this.element.addEventListener(event,f)
      
      return this
    }
    
    css(_style={}){
      for(let prop in _style){
        if(_style[prop] instanceof Function){
          this.element.style[prop]=_style[prop]()
        }
        
        if(typeof _style[prop]==="number"){
          this.element.style[prop]=_style[prop]+"px"
        }
        
        this.element.style[prop]=_style[prop]
      }
      return this
    }
    
    init(){
    this.start = !this.start
    c.log(Anime.TYPE)
    
    return this
    }
    
    
  }
 
export {Anime}


 function _checkElementWithinData(data){
   let d= data.element
   
    if(d===null){
    try{
      let _cnv= dc("canvas")
      _cnv.width= _W.innerHeight/2
      _cnv.height=_W.innerHeight/2
      _cnv.style.backgroundColor="#000"
      body.append(_cnv)
      this.element=dq("canvas")
      
      Anime.TWO_D=true
    }
    catch(err){
      c.warn(err)
    }
 }

   else if(data.element){
     this.element= dq(data.element)
   }
   return true
 }

function _warningAboutChanges(){
   c.warn("Anime is still under developement\n\t By Nonso Martin and CO\n\v stay updated...")
 }
 
