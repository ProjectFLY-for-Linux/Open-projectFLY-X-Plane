use xplm::plugin::{Plugin, PluginInfo};

use xplm::xplane_plugin;

use xplm::data::borrowed::{DataRef, FindError};
use xplm::data::{ReadOnly, ReadWrite, DataRead, ArrayRead, StringRead};
use xplm::flight_loop::{FlightLoop, FlightLoopCallback, LoopState};
use std::time::Duration;
use pfly_rust::*;
use socket2::Socket;
use xplm::draw::Phase;
use std::borrow::Borrow;

struct pFlyPlugin {
    flight_loop: Option<FlightLoop>,
}

impl Plugin for pFlyPlugin {
    type Error = FindError;
    fn start() -> Result<Self, Self::Error> {
        let mut plugin = pFlyPlugin {
            flight_loop: None,
        };
        Ok(plugin)
    }

    fn enable(&mut self) -> Result<(), Self::Error> {
        let pfly_socket = init();

        let altitude: DataRef<f32, ReadOnly> = DataRef::find("sim/flightmodel/position/elevation")?;
        let agl: DataRef<f32, ReadOnly> = DataRef::find("sim/flightmodel/position/y_agl")?;
        let groundspeed: DataRef<f32, ReadOnly> = DataRef::find("sim/flightmodel/position/groundspeed")?;
        let ias: DataRef<f32, ReadOnly> = DataRef::find("sim/flightmodel/position/indicated_airspeed")?;
        let headingTrue: DataRef<f32, ReadOnly> = DataRef::find("sim/flightmodel/position/psi")?;
        let headingMagnetic: DataRef<f32, ReadOnly> = DataRef::find("sim/flightmodel/position/mag_psi")?;
        let latitude: DataRef<f64, ReadOnly> = DataRef::find("sim/flightmodel/position/latitude")?;
        let longitude: DataRef<f64, ReadOnly> = DataRef::find("sim/flightmodel/position/longitude")?;
        let verticalSpeed: DataRef<f32, ReadOnly> = DataRef::find("sim/flightmodel/position/vh_ind_fpm")?;
        let pitch: DataRef<f32, ReadOnly> = DataRef::find("sim/flightmodel/position/theta")?;
        let gForce: DataRef<f32, ReadOnly> = DataRef::find("sim/flightmodel/forces/g_nrml")?;
        let roll: DataRef<f32, ReadOnly> = DataRef::find("sim/flightmodel/position/phi")?;
        let transponder: DataRef<i32, ReadOnly> = DataRef::find("sim/cockpit/radios/transponder_code")?;
        let isOnGround: DataRef<bool, ReadOnly> = DataRef::find("sim/flightmodel/failures/onground_any")?;
        let isSlew: DataRef<bool, ReadOnly> = DataRef::find("sim/operation/prefs/replay_mode")?;
        let isPaused: DataRef<bool, ReadOnly> = DataRef::find("sim/time/paused")?;
        let fps: DataRef<f32, ReadOnly> = DataRef::find("sim/time/gpu_time_per_frame_sec_approx")?;
        let fuel: DataRef<f32, ReadOnly> = DataRef::find("sim/aircraft/weight/acf_m_fuel_tot")?;

        let mut loop_handler =  FlightLoop::new(move |loop_state: &mut LoopState| {
            println!("Getting X-Plane data...");
            if send_message(pfly_socket.try_clone().unwrap(), PflyIpcData{
                altitude: (altitude.get() * 3.2808399) as i32,
                agl: agl.get() as i32,
                groundspeed: (groundspeed.get() * 1.943844) as i32,
                ias: ias.get() as i32,
                headingTrue: headingTrue.get() as i32,
                headingMagnetic: headingMagnetic.get() as i32,
                latitude: latitude.get(),
                longitude: longitude.get(),
                verticalSpeed: verticalSpeed.get() as i32,
                landingVerticalSpeed: 0,
                gForce: gForce.get() as i32,
                fuel: (fuel.get() /  2.20462262) as i32,
                transponder: transponder.get(),
                bridgeType: 3,
                isOnGround: isOnGround.get(),
                isSlew: isSlew.get(),
                isPaused: isPaused.get(),
                pitch: pitch.get() as i32,
                roll: roll.get() as i32,
                time: 0,
                fps: fps.get() as i32,
                aircraftType: ""
            }){
                println!("Sent to projectFLY")
            } else {
                println!("Failed to send data to projectFLY")
            }
        });
        loop_handler.schedule_after(Duration::from_millis(1000));
        self.flight_loop = Some(loop_handler);
        Ok(())
    }

    fn disable(&mut self) {
        self.flight_loop = None;
    }

    fn info(&self) -> PluginInfo {
        PluginInfo {
            name: "Open projectFLY X-Plane".into(),
            signature: "gq.skye.pflyopenxplane".into(),
            description: "An open source replacement for the projectFLY X-Plane plugin.".into(),
        }
    }
}

xplane_plugin!(pFlyPlugin);
