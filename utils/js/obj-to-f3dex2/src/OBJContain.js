function parsedOBJ(){
  this.Vertices = []
  this.VTex     = []
  this.VNormals = []
  this.Faces    = []
  this.Comments = []
  this.Unknown  = []
}

parsedOBJ.prototype.linkFace = function( face ){
  // get a component array of
  // the vertices, vtexture, and vnormal arrays
  const coms = [ this.Vertices, this.VTex, this.VNormals ];

  const linkedVertices = face.initVerts.map( vertex =>{
    const mapped = vertex.map( (component,i) =>{

      if(!isNaN(component)){
        return component > 0 ?
            (coms[i][component-1]) :
            (coms[i][coms[i].length - component]);

      } else {
        return null
      }
    })

    return mapped
  })

  face.setVertices(linkedVertices)

  return this
}

module.exports = parsedOBJ
