const {Vertex_Normal, Vertex_Color} = require('./objects/f3dex.js');
const forceEnum = require('./enum/force-vertex.js');

/**
* Take an input tupple of [v, vt, vn] and convert to the N64's
* vertex color or vertex normal struct
* @param {Array} objV   - obj vertex tupple [Vertex, VTexture, VNormal]
* @param {Material} mtl - Material object for objV vertex tupple
* @param {enum} force   - force normal or color vertices, if needed
* @return {f3dex.Vertex} - either Vetrex_Color or Vertex_Normal
**/

function toF3DEXVtx(objV, mtl, force = forceEnum.none) {
  let [v, vt, vn] = objV,
      vertex;

  if ( vn !== null || force === forceEnum.normal && force !== forceEnum.color ) {
    vertex = new Vertex_Normal();

    vertex.setPosition(v.x, v.y, v.z)
          .setNormal(vn.i, vn.j, vn.k)
          .setAlpha( mtl['alpha'], true )
          //.setTextCoor(s,t) // future addition (in rust)
  } else {
    let {r, g, b} = mtl['ambient'],
                a = mtl['alpha'];

    vertex = new Vertex_Color();


    vertex.setPosition(v.x, v.y, v.z)
          //.setTextCoor(s,t)
          .setColor(r, g, b, a)
          .color.to8bit();
  }

  return vertex
}

module.exports = toF3DEXVtx;
