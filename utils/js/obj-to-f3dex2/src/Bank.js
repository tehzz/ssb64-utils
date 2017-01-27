function Bank(id){
  this.id = id;
  this.length = 0;
  this.full = false;
  this.vertices = []
  this.faces    = []
}

Bank.prototype.find = function( arr, value ){
  return arr.indexOf(value)
}

Bank.prototype.findVertex = function( vertex ){
  // get just the vertex index and compare to input
  return this.find(this.vertices.map(a => a[0]), vertex)
}

Bank.prototype.addFullVertex = function( fullVert ) {
  this.length = this.vertices.push(fullVert)

  //if( vtex !==null )   this.vtex.push(vtex)
  //if( vnorm !== null ) this.vnormals.push(vnorm)

  return this.length
}

module.exports = Bank
