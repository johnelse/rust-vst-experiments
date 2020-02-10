use std::collections::VecDeque;
use std::vec::Vec;

pub struct NoteTracker {
    polyphony:         usize,
    playing_notes:     VecDeque<u8>,
    extra_notes_count: usize,
    extra_notes:       Vec<u8>,
}

impl NoteTracker {
    pub fn new(polyphony: usize, extra_notes_count: usize) -> NoteTracker {
        NoteTracker {
            polyphony:         polyphony,
            playing_notes:     VecDeque::with_capacity(polyphony),
            extra_notes_count: extra_notes_count,
            extra_notes:       Vec::with_capacity(extra_notes_count),
        }
    }

    fn space_for_playing_note(&self) -> bool {
        self.playing_notes.len() < self.polyphony
    }

    fn space_for_extra_note(&self) -> bool {
        self.extra_notes.len() < self.extra_notes_count
    }

    pub fn note_on(&mut self, note: u8) {
        if self.space_for_playing_note() {
            self.playing_notes.push_back(note);
        }
        else if self.space_for_extra_note() {
            match self.playing_notes.pop_front() {
                Some(popped_note) => {
                    self.extra_notes.push(popped_note);
                    self.playing_notes.push_back(note);
                },
                None => (),
            }
        }
    }

    pub fn note_off(&mut self, note: u8) {
        self.extra_notes  .retain(|&x| x != note);
        self.playing_notes.retain(|&x| x != note);

        if self.space_for_playing_note() {
            match self.extra_notes.pop() {
                Some(popped_note) => {
                    self.playing_notes.push_front(popped_note);
                },
                None => (),
            }
        }
    }

    pub fn get_playing_notes(&self) -> Vec<u8> {
        self.playing_notes.iter().copied().collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_initialise()
    {
        let tracker = NoteTracker::new(4, 4);
        assert_eq!(tracker.playing_notes.len(), 0);
        assert_eq!(tracker.extra_notes.len()  , 0);
    }

    #[test]
    fn test_11_fifo()
    {
        let mut tracker = NoteTracker::new(1, 1);

        tracker.note_on(60);
        assert_eq!(tracker.playing_notes, [60]);
        assert_eq!(tracker.extra_notes,   []);

        tracker.note_on(67);
        assert_eq!(tracker.playing_notes, [67]);
        assert_eq!(tracker.extra_notes,   [60]);

        tracker.note_on(72);
        assert_eq!(tracker.playing_notes, [67]);
        assert_eq!(tracker.extra_notes,   [60]);

        tracker.note_off(72);
        assert_eq!(tracker.playing_notes, [67]);
        assert_eq!(tracker.extra_notes,   [60]);

        tracker.note_off(67);
        assert_eq!(tracker.playing_notes, [60]);
        assert_eq!(tracker.extra_notes,   []);

        tracker.note_off(60);
        assert_eq!(tracker.playing_notes, []);
        assert_eq!(tracker.extra_notes,   []);
    }

    #[test]
    fn test_11_lifo()
    {
        let mut tracker = NoteTracker::new(1, 1);

        tracker.note_on(60);
        assert_eq!(tracker.playing_notes, [60]);
        assert_eq!(tracker.extra_notes,   []);

        tracker.note_on(67);
        assert_eq!(tracker.playing_notes, [67]);
        assert_eq!(tracker.extra_notes,   [60]);

        tracker.note_on(72);
        assert_eq!(tracker.playing_notes, [67]);
        assert_eq!(tracker.extra_notes,   [60]);

        tracker.note_off(60);
        assert_eq!(tracker.playing_notes, [67]);
        assert_eq!(tracker.extra_notes,   []);

        tracker.note_off(67);
        assert_eq!(tracker.playing_notes, []);
        assert_eq!(tracker.extra_notes,   []);

        tracker.note_off(72);
        assert_eq!(tracker.playing_notes, []);
        assert_eq!(tracker.extra_notes,   []);
    }
}
