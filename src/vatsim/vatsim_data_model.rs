use serde::{Serialize, Deserialize};
use serde_json::from_str;
use rusoto_dynamodb::PutItemInput;

#[derive(Serialize, Deserialize, Debug)]
struct ParsedVatsimData {
    pub general: GeneralData,
    pub pilots: Vec<ParsedPilotData>,
    pub controllers: Vec<ControllerData>,
    pub atis: Vec<AtisData>,
    pub servers: Vec<ServerData>,
    pub prefiles: Vec<PrefileData>,
    pub facilities: Vec<FacilitiesData>,
    pub ratings: Vec<RatingsData>,
    pub pilot_ratings: Vec<PilotRatingsData>,
}

impl ParsedVatsimData {
    pub fn convert(self) -> VatsimData {
        let update = self.general.update.clone();
        let parsed_pilots = self.pilots;
        let pilots: Vec<PilotData> = parsed_pilots.into_iter().map(|pilot| pilot.convert(update.clone())).collect();
        VatsimData {
            update: self.general.update.clone(),
            general: self.general,
            pilots,
            controllers: self.controllers,
            atis: self.atis,
            servers: self.servers,
            prefiles: self.prefiles,
            facilities: self.facilities,
            ratings: self.ratings,
            pilot_ratings: self.pilot_ratings,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VatsimData {
    pub update: String,
    pub general: GeneralData,
    pub pilots: Vec<PilotData>,
    pub controllers: Vec<ControllerData>,
    pub atis: Vec<AtisData>,
    pub servers: Vec<ServerData>,
    pub prefiles: Vec<PrefileData>,
    pub facilities: Vec<FacilitiesData>,
    pub ratings: Vec<RatingsData>,
    pub pilot_ratings: Vec<PilotRatingsData>,
}

impl VatsimData {
    pub fn from_text(data: String) -> Self {
        let parsed_data: ParsedVatsimData = from_str(&data).expect("Couldnt parse data");
        parsed_data.convert()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GeneralData {
    pub version: f32,
    pub reload: f32,
    pub update: String,
    pub update_timestamp: String,
    pub connected_clients: u32,
    pub unique_users: u32,
}

impl GeneralData {
    pub fn to_dynamo_item(self) -> Result<serde_dynamo::Item, serde_dynamo::Error> {
        serde_dynamo::to_item(self)
    }

    pub fn generate_put_item(item: serde_dynamo::Item) -> rusoto_dynamodb::PutItemInput {
        PutItemInput {
            table_name: "vatsim_data_general".to_string(),
            item,
            ..PutItemInput::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ParsedPilotData {
    pub cid: u32,
    pub name: String,
    pub callsign: String,
    pub server: String,
    pub pilot_rating: i32,
    pub latitude: f32,
    pub longitude: f32,
    pub altitude: i32,
    pub groundspeed: i32,
    pub transponder: String,
    pub heading: u16,
    pub qnh_i_hg: f32,
    pub qnh_mb: i32,
    pub flight_plan: Option<FlightPlanData>,
    pub logon_time: String,
    pub last_updated: String,
}

impl ParsedPilotData {
    pub fn convert(self, update: String) -> PilotData {
        PilotData {
            update,
            cid: self.cid,
            name: self.name,
            callsign: self.callsign,
            server: self.server,
            pilot_rating: self.pilot_rating,
            latitude: self.latitude,
            longitude: self.longitude,
            altitude: self.altitude,
            groundspeed: self.groundspeed,
            transponder: self.transponder,
            heading: self.heading,
            qnh_i_hg: self.qnh_i_hg,
            qnh_mb: self.qnh_mb,
            flight_plan: self.flight_plan,
            logon_time: self.logon_time,
            last_updated: self.last_updated,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PilotData {
    pub update: String,
    pub cid: u32,
    pub name: String,
    pub callsign: String,
    pub server: String,
    pub pilot_rating: i32,
    pub latitude: f32,
    pub longitude: f32,
    pub altitude: i32,
    pub groundspeed: i32,
    pub transponder: String,
    pub heading: u16,
    pub qnh_i_hg: f32,
    pub qnh_mb: i32,
    pub flight_plan: Option<FlightPlanData>,
    pub logon_time: String,
    pub last_updated: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FlightPlanData {
    pub flight_rules: String,
    pub aircraft: String,
    pub aircraft_faa: String,
    pub aircraft_short: String,
    pub departure: String,
    pub arrival: String,
    pub alternate: String,
    pub cruise_tas: String,
    pub altitude: String,
    pub deptime: String,
    pub enroute_time: String,
    pub fuel_time: String,
    pub remarks: String,
    pub route: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ControllerData {
    pub cid: u32,
    pub name: String,
    pub callsign: String,
    pub frequency: String,
    pub facility: i32,
    pub rating: i32,
    pub server: String,
    pub visual_range: i32,
    pub text_atis: Option<Vec<String>>,
    pub last_updated: String,
    pub logon_time: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AtisData {
    pub cid: u32,
    pub name: String,
    pub callsign: String,
    pub frequency: String,
    pub facility: i32,
    pub rating: i32,
    pub server: String,
    pub visual_range: i32,
    pub atis_code: Option<String>,
    pub text_atis: Option<Vec<String>>,
    pub last_updated: String,
    pub logon_time: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerData {
    pub ident: String,
    pub hostname_or_ip: String,
    pub location: String,
    pub name: String,
    pub clients_connection_allowed: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PrefileData {
    pub cid: u32,
    pub name: String,
    pub callsign: String,
    pub flight_plan: Option<FlightPlanData>,
    pub last_updated: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FacilitiesData {
    pub id: i32,
    pub short: String,
    pub long: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RatingsData {
    pub id: i32,
    pub short: String,
    pub long: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PilotRatingsData {
    pub id: i32,
    pub short: Option<String>,
    pub long: Option<String>,
}

#[derive(Deserialize)]
pub struct VatsimStatusData {
    pub data: VatsimStatusDataData,
    pub user: Vec<String>,
    pub metar: Vec<String>,
}

#[derive(Deserialize)]
pub struct VatsimStatusDataData {
    pub v3: Vec<String>,
    pub transceivers: Vec<String>,
    pub servers: Vec<String>,
}