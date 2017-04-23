function Triangle(){
  this.objV = [];
  this.bank = null;
  this.bankVertices = [];
}

Triangle.prototype = {
  addVertex: function(v){
    this['objV'].push(v)

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
