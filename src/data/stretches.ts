export interface Stretch {
  id: string
  name: string
  description: string
  benefits: string
  durationSeconds: number
  imageUrl?: string
}

export const STRETCH_LIBRARY: Stretch[] = [
  {
    id: 'neck-stretch',
    name: 'Neck Stretch',
    description: 'Gently tilt your head towards your shoulder, holding the opposite arm down. Switch sides.',
    benefits: 'Relieves neck tension from looking at screens.',
    durationSeconds: 30,
  },
  {
    id: 'wrist-extension',
    name: 'Wrist Extension',
    description: 'Extend your arm in front of you, palm up. Use your other hand to gently pull your fingers down towards the floor.',
    benefits: 'Prevents repetitive strain injury in the wrists.',
    durationSeconds: 30,
  },
  {
    id: 'seated-twist',
    name: 'Seated Twist',
    description: 'While seated, place your right hand on your left knee and gently twist your torso to the left. Switch sides.',
    benefits: 'Improves spinal mobility and relieves lower back stiffness.',
    durationSeconds: 40,
  },
  {
    id: 'shoulder-shrug',
    name: 'Shoulder Shrug',
    description: 'Raise both shoulders up towards your ears, hold for 5 seconds, then roll them back and down.',
    benefits: 'Releases tension in the upper back and shoulders.',
    durationSeconds: 20,
  },
  {
    id: 'eye-palming',
    name: 'Eye Palming',
    description: 'Rub your hands together to generate heat, then gently cup your palms over your closed eyes.',
    benefits: 'Relaxes eye muscles and reduces digital eye strain.',
    durationSeconds: 30,
  },
]
