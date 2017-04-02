function Triangle(){
  this.verticies = [];
  this.bank = null;
  this.bankVertices = [];
}

Triangle.prototype = {
  addVertex: function(v){
    this['verticies'].push(v)

    return this
  },
  setBank: function(id){
    this.bank = id;
    return this
  },
  bankVertex: function(loc){
    this.bankVertices.push(loc)
    return this
  }
}

module.exports = Triangle
