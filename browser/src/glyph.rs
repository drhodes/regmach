impl Glyph {
    pub fn new(scale: f32, color: rdt::Color, rt_glyph&rusttype::PositionedGlyph<'_>) -> Result<Glyph, String> {
        let mut rows = vec![];
        let mut cur_row = vec![];
        let mut cur_group: Vec<(f32, f32, f32)> = vec![];
        let mut last_v = None;
        let mut last_y = None;

        let mut verts = vec![];

        glyph.draw(|x, y, v| {
            verts.push((x, y, v));
        });


        for (x, y, v) in verts {
            let scale = 0.01;
            let x = -((x as i32 + bb.min.x) as f32 * scale);
            let y = -((y as i32 + bb.min.y) as f32 * scale);

            if v > 0.3 {
                // v should be in the range 0.0 to 1.0

                if Some(y) != last_y {
                    // start a new row
                    cur_row.push(cur_group);
                    rows.push(cur_row);
                    cur_group = vec![];
                    cur_row = vec![vec![(x, y, v)]];
                    last_v = None;
                    last_y = Some(y);
                } else if last_v == None {
                    // if last_v is None, then append this
                    // pixel to the current group. and set
                    // last_v to v
                    last_v = Some(v);
                } else if last_v == Some(v) {
                    // else if v is the same as last_v then
                    // push this pixel onto the current group
                    cur_group.push((x, y, v));
                } else {
                    // else v is different so start a new group.
                    cur_row.push(cur_group);
                    cur_group = vec![(x, y, v)];
                    last_v = Some(v);
                }
            } else {
                // v is invisible, so start a new group
                // and set last_v to None
                cur_row.push(cur_group);
                rows.push(cur_row);
                cur_group = vec![];
                last_v = None;
                last_y = Some(y);
            }
        }
        /*
        // need to draw two small triangles per pixel.
        // or use different mesh routines with four points using gl_fan.
        // but for now 6 points per font pixel.
        // what coordinate system is being used?
        let bl = [x, y, 0.0];
        let br = [x + scale, y, 0.0];
        let tl = [x, y + scale, 0.0];
        let tr = [x + scale, y + scale, 0.0];

        let co = [red, green, blue, v];

        // this is inefficient, TODO (just send one
        // color per triangle)

        // triangle 1: counter clockwise: bl br tl
        text_verts.extend(bl.iter());
        colors.extend(co.iter());
        text_verts.extend(br.iter());
        colors.extend(co.iter());
        text_verts.extend(tl.iter());
        colors.extend(co.iter());

        //triangle 2: counter clockwise: tl br tr
        text_verts.extend(tl.iter());
        colors.extend(co.iter());
        text_verts.extend(br.iter());
        colors.extend(co.iter());
        text_verts.extend(tr.iter());
        colors.extend(co.iter());
         */
    }

    let font_mesh = FontMesh::from_verts(&dsp,
                                         text_verts,
                                         colors,
                                         include_str!("../shaders/font-shader.vs"),
                                         include_str!("../shaders/font-shader.fs"))?;

    Ok(Text { color, font_mesh, text: text.to_owned() }
}
    
