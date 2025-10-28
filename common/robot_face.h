#ifndef ROBOT_FACE_H
#define ROBOT_FACE_H

/**
 * Common interface for robot face implementations
 *
 * Robot Face Design Specifications:
 * - Canvas: 800x600 pixels
 * - Eyes: Two circles at (250, 200) and (550, 200), radius 60px
 * - Pupils: Black circles, radius 40px (animated during blink)
 * - Mouth: Bezier curve from (300, 400) to (500, 400)
 *   - Control point Y varies with emotion: 430 (happy), 400 (neutral), 370 (sad)
 * - Blink: Smooth animation every 3 seconds (200ms duration)
 */

// Emotion values: 0.0 (sad) to 1.0 (happy)
// Blink progress: 0.0 (eyes open) to 1.0 (eyes closed)

#endif // ROBOT_FACE_H
