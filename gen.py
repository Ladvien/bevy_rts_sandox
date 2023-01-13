

from random import randint

for i in range(30):
    
    desired = randint(0, 360)
    current = randint(0, 360)
    
    
    print(f""" 
    #[test]
    fn rotate_cw_current_{current}_desired_{desired}() {{
        let desired = {desired}.0;
        let current = {current}.0;
        let rotation_direction = get_heading(desired, current);
            assert!(matches!(rotation_direction, RotationDirection::CCW))
        }}
          """)