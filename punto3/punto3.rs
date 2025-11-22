struct Sensores {
    gps: (f64, f64, f32), // lat, lon, alt
    inerciales: (f32, f32, f32), // pitch, roll, yaw
    comandos_piloto: (f32, f32, f32, f32), // pitch, roll, yaw, throttle
}

struct Procesamiento {
    limites: (f32, f32, f32), // pitch_max, roll_max, alt_min
}

struct Actuadores {
    superficies: (f32, f32, f32), // alerones, elevadores, timon
    motores: f32,
}

impl Procesamiento {
    fn calcular_comandos(&self, sens: &Sensores) -> (f32, f32, f32, f32) {
        // NUCLEO CONTROL - Zona Critica
        let correccion_pitch = -sens.inerciales.0 * 0.5;
        let correccion_roll = -sens.inerciales.1 * 0.5;
        (correccion_roll, correccion_pitch, 0.0, sens.comandos_piloto.3)
    }
    
    fn verificar_seguridad(&self, sens: &Sensores) -> Result<(), String> {
        // SISTEMA SEGURIDAD - Zona Critica
        if sens.inerciales.0.abs() > self.limites.0 {
            return Err("PITCH EXCEDE LIMITE".to_string());
        }
        if sens.inerciales.1.abs() > self.limites.1 {
            return Err("ROLL EXCEDE LIMITE".to_string());
        }
        if sens.gps.2 < self.limites.2 {
            return Err("ALTITUD DEMASIADO BAJA".to_string());
        }
        Ok(())
    }
    
    fn calcular_trayectoria(&self, pos_actual: (f64, f64, f32)) -> (f64, f64, f32) {
        // CALCULO TRAYECTORIA - Zona Sensible
        (pos_actual.0 + 0.01, pos_actual.1 + 0.01, pos_actual.2)
    }
}

fn main() {
    println!("SISTEMA CONTROL VUELO");
    
    let sensores = Sensores {
        gps: (40.7128, -74.0060, 1000.0),
        inerciales: (5.0, 2.0, 0.0),
        comandos_piloto: (0.0, 0.0, 0.0, 0.8),
    };
    
    let procesamiento = Procesamiento {
        limites: (30.0, 45.0, 500.0),
    };
    
    // CICLO DE CONTROL PRINCIPAL
    match procesamiento.verificar_seguridad(&sensores) {
        Ok(()) => {
            let comandos = procesamiento.calcular_comandos(&sensores);
            let nueva_trayectoria = procesamiento.calcular_trayectoria(sensores.gps);
            
            let actuadores = Actuadores {
                superficies: (comandos.0, comandos.1, comandos.2),
                motores: comandos.3,
            };
            
            println!("Control exitoso");
            println!("Comandos: A={:.2}, E={:.2}, T={:.2}, M={:.2}", 
                     actuadores.superficies.0, actuadores.superficies.1, 
                     actuadores.superficies.2, actuadores.motores);
            println!("Nueva trayectoria: ({:.4}, {:.4}, {})", 
                     nueva_trayectoria.0, nueva_trayectoria.1, nueva_trayectoria.2);
        },
        Err(error) => {
            println!("Error critico: {}", error);
        }
    }
}
