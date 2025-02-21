#import "myscreensaverView.h"

@implementation myscreensaverView {
    NSPoint ballPosition;  // Current position of the ball
    NSSize ballVelocity;   // Velocity of the ball
    CGFloat ballRadius;    // Radius of the ball
}

- (instancetype)initWithFrame:(NSRect)frame isPreview:(BOOL)isPreview
{
    self = [super initWithFrame:frame isPreview:isPreview];
    if (self) {
        [self setAnimationTimeInterval:1.0 / 60.0]; // 60 FPS
        ballRadius = 20.0; // Ball radius
        ballPosition = NSMakePoint(NSWidth(frame) / 2, NSHeight(frame) / 2); // Start in the center
        
        // Set velocity based on screen size
        CGFloat screenDiagonal = sqrt(pow(NSWidth(frame), 2) + pow(NSHeight(frame), 2));
        CGFloat speed = screenDiagonal / 300.0; // Adjust divisor for desired speed
        ballVelocity = NSMakeSize(speed, speed); // Initial velocity
    }
    return self;
}

- (void)startAnimation
{
    [super startAnimation];
}

- (void)stopAnimation
{
    [super stopAnimation];
}

- (void)drawRect:(NSRect)rect
{
    [super drawRect:rect];

    // Clear the screen with a black background
    [[NSColor blackColor] set];
    NSRectFill(rect);

    // Draw the ball
    [[NSColor redColor] set];
    NSRect ballRect = NSMakeRect(ballPosition.x - ballRadius, ballPosition.y - ballRadius, ballRadius * 2, ballRadius * 2);
    NSBezierPath *ballPath = [NSBezierPath bezierPathWithOvalInRect:ballRect];
    [ballPath fill];
}

- (void)animateOneFrame
{
    // Update ball position
    ballPosition.x += ballVelocity.width;
    ballPosition.y += ballVelocity.height;

    // Bounce off walls
    if (ballPosition.x - ballRadius <= 0 || ballPosition.x + ballRadius >= NSWidth(self.bounds)) {
        ballVelocity.width = -ballVelocity.width;
        ballPosition.x = MAX(ballRadius, MIN(ballPosition.x, NSWidth(self.bounds) - ballRadius));
    }
    if (ballPosition.y - ballRadius <= 0 || ballPosition.y + ballRadius >= NSHeight(self.bounds)) {
        ballVelocity.height = -ballVelocity.height;
        ballPosition.y = MAX(ballRadius, MIN(ballPosition.y, NSHeight(self.bounds) - ballRadius));
    }

    // Request to redraw
    [self setNeedsDisplay:YES];
}

- (BOOL)hasConfigureSheet
{
    return NO; // No configuration sheet for this example
}

- (NSWindow *)configureSheet
{
    return nil;
}

@end
