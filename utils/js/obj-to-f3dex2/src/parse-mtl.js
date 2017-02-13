const Material = require('./Material.js')

function parse([p, ...mtls]) {

  mtls.forEach( mtl => {
    let curMtl;

    mtl.split('\n').forEach( line => {
      const keyword = line.split(/\b/)[0].trim(),
            values  = line.split(/\s/).slice(1)

      let colors;

      switch ( keyword ) {
        case '': break;
        // comment
        case '#': break;

        case 'newmtl':
          let name = values.pop().trim()
          // check for already named material...?
          p['Materials'].set(name, new Material(name))
          curMtl = p['Materials'].get(name)
        break;

        case 'Ns':
          let coef = Number.parseFloat(values.pop())
          curMtl.setNs(coef);
        break;

        case 'Ka':
          colors = values.map( i => Number.parseFloat(i) )
          curMtl.setKa(colors)
        break;

        case 'Kd':
          colors = values.map( i => Number.parseFloat(i) )
          curMtl.setKd(colors)
        break;

        case 'Ks':
          colors = values.map( i => Number.parseFloat(i) )
          curMtl.setKs(colors)
        break;

        case 'Ke':
          colors = values.map( i => Number.parseFloat(i) )
          curMtl.setKe(colors)
        break;

        case 'Ni':
          let density = Number.parseFloat(values.pop())
          curMtl.setDensity(density)
        break;

        case 'd':
          let alpha = Number.parseFloat(values.pop())
          curMtl.setAlpha(alpha)
        break;

        case 'illum':
          console.log("Implement illumination enum...")
        break;

        case 'map_Kd':
          console.log('Implement texture file reading...')
        break;

        default:
          console.log(`Unknown mtl command '${keyword}'`)
      }
    })
  })
  return p
}

module.exports = parse;
