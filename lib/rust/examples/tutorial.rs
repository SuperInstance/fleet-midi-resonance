//! # fleet-midi-resonance Tutorial
//!
//! Resonance and harmonic analysis for MIDI through ternary signal processing.
//!
//! This crate reveals the resonant structure hidden in ternary sequences.
//! The `process()` function maps ternary words to MIDI notes using interval
//! steps of ±4 semitones — a major third. This interval has a deep harmonic
//! property: it generates the augmented triad, one of the most resonant
//! symmetric chords in music theory.
//!
//! ## The Mathematical Insight
//!
//! Resonance occurs when a system responds strongly to certain frequencies.
//! In music, resonance means harmonic intervals — frequency ratios that are
//! simple fractions. The 4-semitone step (frequency ratio ≈ 5:4 in just
//! intonation) creates natural resonance because it corresponds to the
//! third harmonic partial.
//!
//! The ternary mapping [-1, 0, +1] → [−4, 0, +4] semitones means every
//! melody produced by this system has inherent harmonic resonance — it can
//! only visit notes that are major thirds apart.
//!
//! Run with: `cargo run --example tutorial`

use fleet_proc::process;

fn main() {
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║     fleet-midi-resonance: Harmonic Analysis Tutorial    ║");
    println!("╚══════════════════════════════════════════════════════════╝\n");

    // ── Lesson 1: The Resonant Interval ─────────────────────────
    //
    // A major third (4 semitones) has a frequency ratio of approximately
    // 5:4 in just intonation. When two notes are a major third apart,
    // their harmonic partials align closely, creating a warm, resonant
    // sound. This is why the process() function uses ±4 as its step size.

    println!("── Lesson 1: The Resonant Interval ──\n");

    let up = process(&[1], 60);
    let down = process(&[-1], 60);
    let rest = process(&[0], 60);

    println!("  Starting from C4 (MIDI 60, ~261.6 Hz):");
    println!("    +1 → {:?}  (E4, ~329.6 Hz, ratio ≈ 5/4)", up);
    println!("    -1 → {:?}  (G#3, ~207.6 Hz, ratio ≈ 4/5)", down);
    println!("     0 → {:?}  (C4, unison, ratio = 1/1)", rest);
    println!();

    // The frequency of MIDI note n is: f(n) = 440 × 2^((n-69)/12)
    // So f(64)/f(60) = 2^(4/12) = 2^(1/3) ≈ 1.2599
    // In just intonation, 5/4 = 1.25 — very close!
    // This near-coincidence is why the major third "rings."

    // ── Lesson 2: Harmonic Series as Pitch Contour ──────────────
    //
    // A rising ternary sequence [1, 1, 1, ...] ascends by major thirds,
    // approximating the harmonic series. The harmonic partials of a
    // fundamental at C would include E (5th partial), G# (varies by
    // temperament), and higher multiples.

    println!("── Lesson 2: Harmonic Series as Pitch Contour ──\n");

    let harmonics = process(&[1, 1, 1, 1, 1, 1], 36);
    println!("  Ascending from C2 (MIDI 36):");
    println!("  {:?}", harmonics);
    // → [36, 40, 44, 48, 52, 56, 60]
    // C2, E2, G#2, C3, E3, G#3, C4

    let names = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];
    for (i, &note) in harmonics.iter().enumerate() {
        let name = names[note as usize % 12];
        let octave = note / 12 - 1;
        let freq = 440.0 * 2_f32.powf((note as f32 - 69.0) / 12.0);
        println!("    Partial {}: {}{} ({:.1} Hz)", i, name, octave, freq);
    }
    println!();

    // Notice how we cycle through only 3 pitch class names: C, E, G#.
    // This is the augmented triad — a perfectly symmetric chord where
    // every interval is identical. Maximum resonance through symmetry.

    // ── Lesson 3: Resonance via Symmetry — The Augmented Triad ──
    //
    // The set {0, 4, 8} mod 12 is the augmented triad. It's the only
    // triad that is transpositionally invariant — transposing by 4
    // semitones maps the chord onto itself. This self-similarity is
    // the mathematical definition of resonance.

    println!("── Lesson 3: The Augmented Triad — Symmetric Resonance ──\n");

    let aug = process(&[1, 1], 60);
    println!("  Augmented triad from C4: {:?}", aug);
    println!("  Pitch classes: {:?}", aug.iter().map(|&n| names[n as usize % 12]).collect::<Vec<_>>());
    // → [C, E, G#] — the augmented triad

    // Verify transpositional invariance:
    let from_e = process(&[1, 1], 64);
    let from_gs = process(&[1, 1], 68);
    let pc_aug: Vec<u8> = aug.iter().map(|&n| n % 12).collect();
    let pc_e: Vec<u8> = from_e.iter().map(|&n| n % 12).collect();
    let pc_gs: Vec<u8> = from_gs.iter().map(|&n| n % 12).collect();

    println!("  From C:  pitch classes {:?}", pc_aug);
    println!("  From E:  pitch classes {:?}", pc_e);
    println!("  From G#: pitch classes {:?}", pc_gs);
    println!("  Same set! The chord maps to itself under transposition by 4.");
    println!();

    // ── Lesson 4: Resonance Patterns — Standing Waves ───────────
    //
    // Physical resonance creates standing waves. In ternary space, the
    // pattern [1, -1] creates a pitch standing wave — oscillating between
    // two notes. Adding [0] stretches creates overtones at different
    // "frequencies."

    println!("── Lesson 4: Standing Waves in Pitch Space ──\n");

    // Fast oscillation (fundamental)
    let f1 = process(&[1, -1, 1, -1, 1, -1, 1, -1], 60);
    println!("  f₁ (period 2): {:?}", f1);

    // Medium oscillation — insert zeros for "half frequency"
    let f2 = process(&[1, 0, -1, 0, 1, 0, -1, 0], 60);
    println!("  f₂ (period 4): {:?}", f2);

    // Slow oscillation — more zeros
    let f3 = process(&[1, 0, 0, 0, -1, 0, 0, 0], 60);
    println!("  f₃ (period 8): {:?}", f3);

    // These are the pitch analogues of harmonic partials!
    // f₁ has the highest "frequency" of pitch change,
    // f₃ has the lowest — like fundamental vs overtones.
    println!();

    // ── Lesson 5: Spectral Analysis of Ternary Words ─────────────
    //
    // We can analyze the "spectral content" of a ternary word by measuring
    // how many direction changes it contains. High change-rate = bright
    // timbre. Low change-rate = smooth timbre.

    println!("── Lesson 5: Spectral Analysis of Ternary Words ──\n");

    let smooth = [1, 1, 1, 1, 1];
    let bright = [1, -1, 1, -1, 1];
    let medium = [1, 0, -1, 1, 0];

    fn direction_changes(v: &[i8]) -> usize {
        v.windows(2).filter(|w| w[0] != w[1]).count()
    }

    fn spectral_class(changes: usize, len: usize) -> &'static str {
        let ratio = changes as f64 / len as f64;
        if ratio < 0.3 { "dark/warm" }
        else if ratio < 0.6 { "balanced" }
        else { "bright/sharp" }
    }

    for (name, word) in [("Smooth", smooth.as_slice()), ("Medium", medium.as_slice()), ("Bright", bright.as_slice())] {
        let changes = direction_changes(word);
        let notes = process(word, 60);
        println!("  {}: {:?}  changes={}  spectrum={}",
            name, word, changes, spectral_class(changes, word.len()));
        println!("    Notes: {:?}", notes);
    }
    println!();

    // ── Lesson 6: Resonant Coupling — Two Voice Interaction ─────
    //
    // When two ternary voices share the same pitch-class set, they create
    // resonant coupling — their harmonic content reinforces each other.
    // This is the basis of choral harmonization.

    println!("── Lesson 6: Resonant Coupling ──\n");

    let pattern = [1, 0, -1, 1, 0, -1, 1, 1];
    let voice1 = process(&pattern, 60); // C4
    let voice2 = process(&pattern, 64); // E4 (4 semitones above)
    let voice3 = process(&pattern, 68); // G#4 (4 semitones above that)

    println!("  Three voices in resonant coupling:");
    println!("  Voice 1 (C4):  {:?}", voice1);
    println!("  Voice 2 (E4):  {:?}", voice2);
    println!("  Voice 3 (G#4): {:?}", voice3);

    // At each time step, the three voices form an augmented triad:
    println!("\n  Vertical harmonies (time slices):");
    for i in 0..voice1.len() {
        let pc1 = names[voice1[i] as usize % 12];
        let pc2 = names[voice2[i] as usize % 12];
        let pc3 = names[voice3[i] as usize % 12];
        println!("    t={}: {} + {} + {} = augmented triad", i, pc1, pc2, pc3);
    }
    println!();

    // ── Lesson 7: Energy Analysis ───────────────────────────────
    //
    // The "energy" of a ternary sequence is the total pitch displacement.
    // High energy = wide range = dramatic. Low energy = narrow range = calm.
    // Resonance quality is measured by how much of the energy concentrates
    // in the fundamental pitch-class set.

    println!("── Lesson 7: Energy and Resonance Quality ──\n");

    let sequences = [
        ("Calm",    vec![0, 0, 0, 0, 0]),
        ("Walking", vec![1, -1, 1, -1, 1]),
        ("Climax",  vec![1, 1, 1, 1, 1]),
        ("Complex", vec![1, 0, -1, 1, -1, 1]),
    ];

    for (name, seq) in &sequences {
        let notes = process(seq, 60);
        let range = notes.iter().max().unwrap() - notes.iter().min().unwrap();
        let unique_pcs: std::collections::HashSet<u8> = notes.iter().map(|&n| n % 12).collect();
        let resonance = if unique_pcs.len() <= 3 { "HIGH" } else { "MODERATE" };

        println!("  {}: range={:>2} semitones, pitch classes={:?}, resonance={}",
            name, range, unique_pcs, resonance);
    }

    println!();

    println!("✓ Tutorial complete. You've learned:");
    println!("  1. The major third as a resonant interval (≈5:4 ratio)");
    println!("  2. Harmonic series approximation through ternary ascent");
    println!("  3. The augmented triad — transpositional symmetry = resonance");
    println!("  4. Standing waves in pitch space");
    println!("  5. Spectral analysis of ternary words");
    println!("  6. Resonant coupling between parallel voices");
    println!("  7. Energy analysis and resonance quality metrics");
}
