use crate::dsp::colors;
use crate::dsp::types as dt;
use crate::dsp::types::Command as C;
use crate::schem::types::*;

impl Schematic {
    pub fn from_display(dsp: Box<dt::Display>) -> Schematic {
        Schematic { entities: vec![], display: dsp }
    }

    pub fn setup_buttons(&mut self) {
        // self.display.exec_cmds(vec!(
        //     C::Button(
        // ))
    }

    // }

    // pub fn render(&self) -> Vec<dsp::Command> {
    //     let mut cmds = self.render_grid();
    //     for e in &self.entities {
    //         cmds.append(&mut e.render());
    //     }
    //     cmds
    // }

    // pub fn render_grid(&self) -> Vec<dsp::Command> {
    //     vec![dsp::Command::SetDrawColor(colors::LIGHT_BLUE)]
    // }

    // pub fn add_wire(&mut self) {
    //     let wire = Wire { id: 0, // TODO: need a nonce generator.
    //                       segments: vec![dsp::Segment::from_coords(10, 10, 40, 10),
    //                                      dsp::Segment::from_coords(40, 10, 40, 40),
    //                                      dsp::Segment::from_coords(40, 40, 60, 40),
    //                                      dsp::Segment::from_coords(60, 40, 60, 80),] };
    //     self.entities.push(box wire)
    // }
    // pub fn add_tool(&mut self) {}
}
