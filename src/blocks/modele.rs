use crate::blocks;
use crate::blocks::material::{self, BlockMaterial};
use crate::components::collider::Collider;
use crate::menu::menu::Assets;
use raylib::ffi;
use raylib::prelude::*;

// Permet d'activer l'autorisation de la comparaison entre valeurs
#[derive(PartialEq, Clone, Copy)]
pub enum BlockType {
    Fixe,
    RotationV,
    RotationH,
    Drag,
    All,
}

// Block avec sa fonction d'implementation
pub struct BlockPrefab {
    pub position: crate::Vector3,
    pub size: crate::Vector3,
    pub block_type: BlockType,
    pub material: material::BlockMaterial,
    pub temp_color: Option<Color>,
    pub collider: Collider,
}

// Implémentation pour un block
impl BlockPrefab {
    pub fn new(
        pos: Vector3,
        size: Option<Vector3>,
        block_type: BlockType,
        material: BlockMaterial,
    ) -> Self {
        let actual_size = size.unwrap_or(Vector3::new(1.0, 1.0, 1.0));
        Self {
            position: pos,
            size: actual_size,
            block_type,
            material: material,
            temp_color: None,
            collider: Collider::with_box_from_size(actual_size.x, actual_size.y, actual_size.z),
        }
    }
}

pub struct GroupBlock {
    pub position: Vector3,
    pub orientation: Quaternion,
    pub children: Vec<blocks::modele::BlockPrefab>,
    pub block_type: BlockType,

    pub target_orientation: Quaternion,
    pub is_rotating: bool,
    pub rotation_progress: f32,

    pub start_pos: Vector3,
    pub end_pos: Vector3,
    pub is_dragging: bool,
    pub drag_timer: f32,
}

// Implémentation pour un groupe de block(s)
impl GroupBlock {
    pub fn new(
        pos: Vector3,
        children: Vec<blocks::modele::BlockPrefab>,
        block_type: BlockType,
    ) -> Self {
        Self {
            position: pos,
            orientation: Quaternion::identity(),
            target_orientation: Quaternion::identity(),
            children,   // Liste des blocks
            block_type, // Type de groupe

            // Rotation
            is_rotating: false,
            rotation_progress: 1.0,

            // Drag
            start_pos: pos,
            end_pos: pos,
            is_dragging: false,
            drag_timer: 0.0,
        }
    }

    /*  Fonction pour créer un block unique avec:
     * [pos] -> Vector3 : Position du block
     * [block_type] -> BlockType : Type du block (Fixe, RotationH)
     * [material] -> Materiel du block (sand, grass)
     */
    pub fn single(pos: Vector3, block_type: BlockType, material: BlockMaterial) -> Self {
        let child = BlockPrefab::new(Vector3::ZERO, None, block_type.clone(), material);
        Self::new(pos, vec![child], block_type)
    }

    /* Fonction qui applique la couleur au groupe
     * [color] -> Color : Couleur du block
     */
    pub fn set_temporary_color(&mut self, color: Color) {
        for child in self.children.iter_mut() {
            child.temp_color = Some(color);
        }
    }

    // Fonction qui change la couleur du block a celui de départ
    pub fn reset_color(&mut self) {
        for child in self.children.iter_mut() {
            child.temp_color = None;
        }
    }

    /* Fonction qui dessine tous les cubes du groupe à chaque appel
     * [d] -> &mut RaylibMode3D<RaylibDrawHandle> : Reçoit le contexte de dessin 3D actif pour envoyer les ordres de rendu
     * [assets] -> &Assets : permet de récupérer la texture des assets
     */
    pub fn draw(&self, d: &mut RaylibMode3D<RaylibDrawHandle>, assets: &Assets) {
        // Calcul de l'orientation du groupe
        let animated_orientation = self
            .orientation
            .slerp(self.target_orientation, self.rotation_progress);
        let mat = animated_orientation.to_matrix();

        let matrix_array: [f32; 16] = [
            mat.m0, mat.m4, mat.m8, mat.m12, mat.m1, mat.m5, mat.m9, mat.m13, mat.m2, mat.m6,
            mat.m10, mat.m14, mat.m3, mat.m7, mat.m11, mat.m15,
        ];

        // Unsafe car [ffi] est écrit en C donc Rust ne peut plus garantir la sécurité de la mémoire
        unsafe {
            raylib::ffi::rlPushMatrix();
            raylib::ffi::rlTranslatef(self.position.x, self.position.y, self.position.z);

            // On applique la rotation du groupe
            raylib::ffi::rlMultMatrixf(matrix_array.as_ptr());

            for child in &self.children {
                let color_to_draw = child.temp_color.unwrap_or(child.material.color);

                let tex = child
                    .material
                    .texture_id
                    .and_then(|id| assets.textures.get(id as usize))
                    .unwrap_or(&assets.textures[0]);

                draw_cube_with_texture(
                    tex,
                    child.position,
                    child.size.x,
                    child.size.y,
                    child.size.z,
                    color_to_draw,
                );

                d.draw_cube_wires(
                    child.position,
                    child.size.x,
                    child.size.y,
                    child.size.z,
                    Color::BLACK,
                );
            }
            raylib::ffi::rlPopMatrix();
        }
    }

    /* Fonction qui est appelée lorsque la souris est sur le cube
     * [rl] -> &RaylibHandle : Représente l'accès direct au moteur de jeu pour lire les entrées
     * [camera] -> &Camera3D : permet d'avoir accès à la position de cubes par rapport a la caméra
     */
    pub fn is_mouse_over(&self, rl: &RaylibHandle, camera: &Camera3D) -> bool {
        let ray = rl.get_screen_to_world_ray(rl.get_mouse_position(), camera);

        for child in &self.children {
            let world_child_pos = self.position + child.position;

            let half_size = child.size * 0.5;
            let bbox = BoundingBox::new(world_child_pos - half_size, world_child_pos + half_size);
            if bbox.get_ray_collision_box(ray).hit {
                return true;
            }
        }
        false
    }

    /* Fonction qui met à jour les animatiosn du cubes
     * [dt] -> f32 : C'est le temps écoulé depuis la dernière image
     */
    pub fn update_animation(&mut self, dt: f32) {
        if self.is_rotating {
            self.rotation_progress += 3.0 * dt; // Vitesse de rotation
            if self.rotation_progress >= 1.0 {
                self.rotation_progress = 1.0;
                self.orientation = self.target_orientation;
                self.is_rotating = false;
            }
            self.sync_colliders()
        }
    }

    pub fn sync_colliders(&mut self) {
        for child in self.children.iter_mut() {
            child.collider.offset = self.position + child.position; // world pos via offset
        }
    }

    /* Fonctionn qui dessine le drag guide
     * [d] -> &mut RaylibMode3D<RaylibDrawHandle> : Reçoit le contexte de dessin 3D actif pour envoyer les ordres de rendu
     */
    pub fn draw_drag_guides(&self, d: &mut RaylibMode3D<RaylibDrawHandle>) {
        if !self.is_dragging {
            return;
        }
        let axis = self.end_pos - self.start_pos;
        let current_v = self.position - self.start_pos;
        let progress = current_v.dot(axis) / axis.dot(axis);

        if progress > 0.5 {
            Color::LIME
        } else {
            Color::WHITE
        };

        unsafe {
            raylib::ffi::rlPushMatrix();
            raylib::ffi::rlTranslatef(self.end_pos.x, self.end_pos.y, self.end_pos.z);
            // On applique la rotation actuelle pour le fantôme
            let mat = self.orientation.to_matrix();

            let matrix_array: [f32; 16] = [
                mat.m0, mat.m4, mat.m8, mat.m12, mat.m1, mat.m5, mat.m9, mat.m13, mat.m2, mat.m6,
                mat.m10, mat.m14, mat.m3, mat.m7, mat.m11, mat.m15,
            ];

            raylib::ffi::rlMultMatrixf(matrix_array.as_ptr());

            for child in &self.children {
                d.draw_cube_wires(
                    child.position,
                    child.size.x,
                    child.size.y,
                    child.size.z,
                    Color::LIME.alpha(0.4),
                );
            }
            raylib::ffi::rlPopMatrix();
        }

        // Dessine les petits points de trajectoire
        let segments = 10;
        for i in 0..=segments {
            let t = i as f32 / segments as f32;
            let dot_pos = self.start_pos.lerp(self.end_pos, t);
            d.draw_sphere(dot_pos, 0.05, Color::WHITE);
        }
    }

    // Si un cube à le drag, alors on peut lui rajouter une position de fin via cette fonction
    pub fn with_end_pos(mut self, end: Vector3) -> Self {
        self.end_pos = end;
        self
    }
}

/* Fonction qui permet d'instancier la texture sur un cube */
fn draw_cube_with_texture(
    tex: &Texture2D,
    position: Vector3,
    width: f32,
    height: f32,
    length: f32,
    color: Color,
) {
    // demi-dimensions pour centrer le cube sur sa position
    let (x, y, z) = (position.x, position.y, position.z);
    let (w, h, l) = (width / 2.0, height / 2.0, length / 2.0);

    // les 8 coins du cube, nommés par position (avant/arrière, haut/bas, gauche/droite)
    let avant_bas_gauche = (x - w, y - h, z + l);
    let avant_bas_droit = (x + w, y - h, z + l);
    let avant_haut_gauche = (x - w, y + h, z + l);
    let avant_haut_droit = (x + w, y + h, z + l);

    let arr_bas_gauche = (x - w, y - h, z - l);
    let arr_bas_droit = (x + w, y - h, z - l);
    let arr_haut_gauche = (x - w, y + h, z - l);
    let arr_haut_droit = (x + w, y + h, z - l);

    unsafe {
        ffi::rlSetTexture(tex.id);
        ffi::rlBegin(ffi::RL_QUADS as i32);
        ffi::rlColor4ub(color.r, color.g, color.b, color.a);

        // chaque face = 4 sommets dans le sens anti-horaire vu de l'extérieur
        // les coordonnées UV vont de (0,0) en haut-gauche à (1,1) en bas-droite

        draw_face(
            avant_bas_gauche,
            avant_bas_droit,
            avant_haut_droit,
            avant_haut_gauche,
        ); // avant
        draw_face(
            arr_bas_droit,
            arr_bas_gauche,
            arr_haut_gauche,
            arr_haut_droit,
        ); // arrière
        draw_face(
            arr_haut_gauche,
            avant_haut_gauche,
            avant_haut_droit,
            arr_haut_droit,
        ); // dessus
        draw_face(
            arr_bas_droit,
            avant_bas_droit,
            avant_bas_gauche,
            arr_bas_gauche,
        ); // dessous
        draw_face(
            avant_bas_droit,
            arr_bas_droit,
            arr_haut_droit,
            avant_haut_droit,
        ); // droite
        draw_face(
            arr_bas_gauche,
            avant_bas_gauche,
            avant_haut_gauche,
            arr_haut_gauche,
        ); // gauche

        ffi::rlEnd();
        ffi::rlSetTexture(0);
    }
}

// dessine un quad avec les UV aux 4 coins standard
unsafe fn draw_face(
    bas_gauche: (f32, f32, f32),
    bas_droit: (f32, f32, f32),
    haut_droit: (f32, f32, f32),
    haut_gauche: (f32, f32, f32),
) {
    ffi::rlTexCoord2f(0.0, 1.0);
    ffi::rlVertex3f(bas_gauche.0, bas_gauche.1, bas_gauche.2);
    ffi::rlTexCoord2f(1.0, 1.0);
    ffi::rlVertex3f(bas_droit.0, bas_droit.1, bas_droit.2);
    ffi::rlTexCoord2f(1.0, 0.0);
    ffi::rlVertex3f(haut_droit.0, haut_droit.1, haut_droit.2);
    ffi::rlTexCoord2f(0.0, 0.0);
    ffi::rlVertex3f(haut_gauche.0, haut_gauche.1, haut_gauche.2);
}
