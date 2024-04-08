use chrono::{Local, Utc}; // Paquete para usar la fecha y tiempo
use sha2::{Digest, Sha256}; // Paquete para dar soporte a SHA-2
use log::{info, warn}; // Paquete para implementar registros o logs
use std::io::Write; // Usamos el trait 'Write' de Rust

// Definimos una variable DIFICULTAD con números 0 de tipo string
const DIFICULTAD: &str = "00000";

// Estructura para la blockchain 
struct Blockchain {
   blocks: Vec<Block>,
}

// Estructura para un bloque independiente de la blockchain 
struct Block {
   id: u64,
   hash: String,
   previous_hash: String,
   timestamp: i64,
   data: String,
   nonce: u64,
}

// Implementamos las tareas para cada bloque de la blockchain 
impl Block {

   // Función para crear un nuevo bloque
   fn new(id: u64, previous_hash: String, data: String) -> Self {
      
      // Usamos el tiempo 
      let timestamp = Utc::now().timestamp();

      // Generamos un nuevo bloque, el cual hereda información del bloque anterior 
      let (hash, nonce) = Self::minar(id, previous_hash.clone(), timestamp, data.clone());

      // Organizamos los tipos de datos que usamos en nuestra función 'new()'
      Self {
         id,
         hash,
         previous_hash,
         timestamp,
         data,
         nonce,
      }
   }
   
   // Función para crear un nuevo hash para el nuevo bloque 
   fn hash(id: u64, previous_hash: String, timestamp: i64, data: String, nonce: u64) -> String {

      // Juntamos los datos de los bloques unificados 
      let datos_bloque_unificados = format!("{}{}{}{}{}", id, previous_hash, timestamp, data, nonce);

      // Creamos una variable mutable llamada 'hash' y actualizamos los datos del siguiente bloque
      let mut hash = Sha256::new();
      hash.update(datos_bloque_unificados);
      format!("{:x}", hash.finalize())

   }

   // Función para minar un bloque
   fn minar(id: u64, previous_hash: String, timestamp: i64, data: String) -> (String, u64) {
      
      // Creamos una variable mutable con valor 0 
      let mut nonce = 0;

      // Loop para minar un bloque 
      loop {

         // Minamos un bloque obteniendo datos del bloque anterior 
         let hash = Self::hash(id, previous_hash.clone(), timestamp, data.clone(), nonce);

         // Si el bloque fue minado correctamente, mostramos un mensaje 
         if hash.as_str().starts_with(DIFICULTAD) {
            info!("El bloque #{} fue minado exitosamente", id);
            return (hash, nonce);
         }

         // Cambiamos el valor del variable nonce 
         nonce += 1;
      }

   }

}

// Implementamos las tareas para la blockchain 
impl Blockchain {

   // Función para crear la blockchain 
   fn new() -> Self {
      Self { blocks: Vec::new() }
   }

   // Función para dar origen a la blockchain 
   fn crear_genesis(&mut self) {

      // Usamos el tiempo 
      let timestamp = Utc::now().timestamp();

      // Vinculamos valores de la implementación 'Block' 
      let (hash, nonce) = Block::minar(
         0,
         String::from("genesis"),
         timestamp,
         String::from("genesis"),
      );

      // Definimos los datos para la blockchain 
      let bloque_genesis = Block {
         id: 0,
         hash,
         previous_hash: String::from("genesis"),
         timestamp,
         data: String::from("genesis"),
         nonce,
      };

      // Creamos el bloque inicial de la blockchain 
      self.blocks.push(bloque_genesis);
      info!("El bloque Génesis se creó con éxito y se agregó a la blockchain.");

   }

   // Función para verificar si el bloque es válido 
   fn validacion_bloque(&self, block: &Block, bloque_anterior: &Block) -> bool {
      
      // Analizamos los datos del bloque 
      if (block.id == bloque_anterior.id + 1)
         && block.hash.starts_with(DIFICULTAD)
         && (block.previous_hash == bloque_anterior.hash)
         && (Block::hash(
            block.id,
            block.previous_hash.clone(),
            block.timestamp,
            block.data.clone(),
            block.nonce,
         ) == block.hash)
      {
         // Mostramos un mensaje para confirmar que el bloque es válido 
         info!("El bloque #{} es válido", block.id);
         return true;
      }

      // Si el bloque no es válido, mostramos un mensaje 
      warn!("El bloque #{} no es válido", block.id);
      false
   }

   // Verificamos si la blockchain es válida 
   fn validacion_cadena(&self) -> bool {

      // Si la blockchain no es válida, mostramos con un mensaje 
      for block_index in 1..self.blocks.len() {
         if !self.validacion_bloque(&self.blocks[block_index], &self.blocks[block_index - 1]) {
            warn!("La blockchain no es válida");
            return false;
         }
      }

      // Si la blockchain sí es válida, mostramos un mensaje 
      info!("La blockchain sí es valida");
      true
    }
   
   // Función para agregar un nuevo bloque a la blockchain 
   fn agregar_bloque(&mut self, block: Block) {

      // Si no hay un bloque para agregar, mostramos un mensaje 
      let bloque_anterior = self
         .blocks
         .last()
         .expect("Debe haber al menos un bloque en la blockchain");

      // Si el bloque es válido lo agregamos a la blockchain 
      // Y si el bloque no es válido, mostramos un mensaje 
      if self.validacion_bloque(&block, bloque_anterior) {
         self.blocks.push(block);
         info!("El bloque se agregó con éxito a la blockchain.");
      } else {
         warn!(
            "El bloque no es válido, no se puede enviar el bloque #{} a la blockchain",
            block.id
         );
      }
   }
}

// Función principal 
fn main() {

   // Mostramos los mensajes en la terminal 
   pretty_env_logger::formatted_timed_builder()
      .format(|buf, record| {
         writeln!(
            buf,
            "{} [{}] - {}",
            Local::now().format("%H:%M:%S"),
            record.level(),
            record.args()
         )
      })
      .filter(None, log::LevelFilter::Info)
      .init();
   
   // Creamos la blockchain 
   let mut blockchain = Blockchain::new();
   blockchain.crear_genesis();
    
   // Ejecutamos las funciones y tareas creadas anteriormente 
   // Mostramos un par de mensajes de rutina 
   loop {
      let bloque_anterior = blockchain
         .blocks
         .last()
         .expect("Debe haber al menos un bloque en la blockchain");
      let nuevo_bloque = Block::new(
         bloque_anterior.id + 1,
         bloque_anterior.hash.clone(),
         String::from("Creación de Una Blockchain con Rust"),
      );

      blockchain.agregar_bloque(nuevo_bloque);

      if blockchain.blocks.len() % 10 == 0 {
         blockchain.validacion_cadena();
     }
   }

}