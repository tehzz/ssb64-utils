function Face( verticies ) {
  this.initVerts = verticies
  this.linkedVerts = null;
  this.vertIDs = null;
}

Face.prototype.setVertices = function( vs ){
  this.linkedVerts = vs

  return this
}

Face.prototype.setVIds = function( vLen, vtLen, vnLen ){
  const lenArr = [ vLen, vtLen, vnLen ];


  this.vertIDs = this.initVerts.map( vertex => {
    return vertex.map( (component,i) =>{
      if( !isNaN(component) ) {
        return component > 0 ?
               component - 1 :    // set to zero index
               lenArr[i] + component // add neg index to current array length
      } else {
        // if no defined component for this vertex
        return null
      }
    })
  })

  return this
}

module.exports = Face;
