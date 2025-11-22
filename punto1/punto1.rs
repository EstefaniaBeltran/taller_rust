fn main() {
    println!("Iniciando demostracion...");
    
    // Stack: datos de tamano fijo
    let stack_var = 42;
    println!("Valor en stack: {}", stack_var);
    
    // Heap: datos de tamano variable
    let heap_owner = String::from("Texto en heap");
    println!("String creado: {}", heap_owner);
    
    // Si intentamos mover el ownership falla:
    // let thief = heap_owner;
    // println!("{}", heap_owner); // Error de compilacion
    
    // Prestamo inmutable funciona
    let prestamo = &heap_owner;
    println!("Prestamo funcionando: {}", prestamo);
    
    // El borrow checker protege contra errores
    let mut datos = vec![1, 2, 3];
    let ref_inmutable = &datos[0];
    
    // Esto seria rechazado por el compilador:
    // datos.push(4); // No permite modificar con referencia activa
    
    println!("Referencia usada: {}", ref_inmutable);
    
    // Despues de usar la referencia, si podemos modificar
    datos.push(4);
    println!("Vector modificado: {:?}", datos);
    
    // Los scopes manejan la liberacion automatica
    {
        let temporal = String::from("Recurso temporal");
        let _recurso_demo = Recurso("recurso_especial");
        println!("Usando recurso temporal: {}", temporal);
    } // Aqui se libera automaticamente
    
    println!("Programa finalizado correctamente");
}

struct Recurso(&'static str);

impl Drop for Recurso {
    fn drop(&mut self) {
        println!("Liberando recurso: {}", self.0);
    }
}