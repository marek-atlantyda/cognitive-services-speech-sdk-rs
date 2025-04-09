use crate::common::{PropertyCollection, PropertyId};
use crate::error::{convert_err, Result};
use crate::ffi::{
    create_pronunciation_assessment_config, pronunciation_assessment_config_apply_to_recognizer,
    pronunciation_assessment_config_get_property_bag, pronunciation_assessment_config_release,
    Pronunciation_Assessment_Grading_System_PronunciationAssessmentGradingSystem_FivePoint,
    Pronunciation_Assessment_Grading_System_PronunciationAssessmentGradingSystem_HundredMark,
    Pronunciation_Assessment_Granularity_PronunciationAssessmentGranularity_FullText,
    Pronunciation_Assessment_Granularity_PronunciationAssessmentGranularity_Phoneme,
    Pronunciation_Assessment_Granularity_PronunciationAssessmentGranularity_Word, SmartHandle,
    SPXPRONUNCIATIONASSESSMENTCONFIGHANDLE, SPXPROPERTYBAGHANDLE,
};

use std::{ffi::CString, mem::MaybeUninit};

use super::SpeechRecognizer;

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u32)]
pub enum PronunciationAssessmentGradingSystem {
    FivePoint =
        Pronunciation_Assessment_Grading_System_PronunciationAssessmentGradingSystem_FivePoint,
    HundredMark =
        Pronunciation_Assessment_Grading_System_PronunciationAssessmentGradingSystem_HundredMark,
}

impl PronunciationAssessmentGradingSystem {
    fn to_string(&self) -> String {
        match self {
            PronunciationAssessmentGradingSystem::FivePoint => "FivePoint".to_string(),
            PronunciationAssessmentGradingSystem::HundredMark => "HundredMark".to_string(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u32)]
pub enum PronunciationAssessmentGranularity {
    Phoneme = Pronunciation_Assessment_Granularity_PronunciationAssessmentGranularity_Phoneme,
    Word = Pronunciation_Assessment_Granularity_PronunciationAssessmentGranularity_Word,
    FullText = Pronunciation_Assessment_Granularity_PronunciationAssessmentGranularity_FullText,
}

impl PronunciationAssessmentGranularity {
    fn to_string(&self) -> String {
        match self {
            PronunciationAssessmentGranularity::Phoneme => "Phoneme".to_string(),
            PronunciationAssessmentGranularity::Word => "Word".to_string(),
            PronunciationAssessmentGranularity::FullText => "FullText".to_string(),
        }
    }
}

pub struct PronunciationAssessmentConfig {
    handle: SmartHandle<SPXPRONUNCIATIONASSESSMENTCONFIGHANDLE>,
    properties: PropertyCollection,
}

impl PronunciationAssessmentConfig {
    pub fn set_property(&mut self, id: PropertyId, value: String) -> Result<()> {
        self.properties.set_property(id, value)
    }

    pub fn get_property(&self, id: PropertyId) -> Result<String> {
        self.properties.get_property(id, "")
    }

    unsafe fn from_handle(handle: SPXPRONUNCIATIONASSESSMENTCONFIGHANDLE) -> Result<Self> {
        unsafe {
            let mut prop_bag_handle: MaybeUninit<SPXPROPERTYBAGHANDLE> = MaybeUninit::uninit();
            let ret = pronunciation_assessment_config_get_property_bag(
                handle,
                prop_bag_handle.as_mut_ptr(),
            );
            convert_err(ret, "PronunciationAssessmentConfig::from_handle error")?;

            let property_bag = PropertyCollection::from_handle(prop_bag_handle.assume_init());

            let result = PronunciationAssessmentConfig {
                handle: SmartHandle::create(
                    "PronunciationAssessmentConfig",
                    handle,
                    pronunciation_assessment_config_release,
                ),
                properties: property_bag,
            };
            Ok(result)
        }
    }

    pub fn create(
        reference_text: &str,
        grading_system: PronunciationAssessmentGradingSystem,
        granularity: PronunciationAssessmentGranularity,
        enable_miscue: bool,
    ) -> Result<Self> {
        let c_reference_text = CString::new(reference_text)?;
        unsafe {
            let mut handle: MaybeUninit<SPXPRONUNCIATIONASSESSMENTCONFIGHANDLE> =
                MaybeUninit::uninit();
            convert_err(
                create_pronunciation_assessment_config(
                    handle.as_mut_ptr(),
                    c_reference_text.as_ptr(),
                    grading_system as u32,
                    granularity as u32,
                    enable_miscue,
                ),
                "PronunciationAssessmentConfig::create error",
            )?;
            PronunciationAssessmentConfig::from_handle(handle.assume_init())
        }
    }

    pub fn apply_to_recognizer(&self, recognizer: &mut SpeechRecognizer) -> Result<()> {
        unsafe {
            convert_err(
                pronunciation_assessment_config_apply_to_recognizer(
                    self.handle.inner(),
                    recognizer.handle.inner(),
                ),
                "PronunciationAssessmentConfig::apply_to_recognizer error",
            )?;
            Ok(())
        }
    }

    pub fn set_reference_text(&mut self, reference_text: String) -> Result<()> {
        self.set_property(
            PropertyId::PronunciationAssessmentReferenceText,
            reference_text,
        )
    }

    pub fn set_grading_system(
        &mut self,
        system: PronunciationAssessmentGradingSystem,
    ) -> Result<()> {
        self.set_property(
            PropertyId::PronunciationAssessmentGradingSystem,
            system.to_string(),
        )
    }

    pub fn set_granularity(
        &mut self,
        granularity: PronunciationAssessmentGranularity,
    ) -> Result<()> {
        self.set_property(
            PropertyId::PronunciationAssessmentGranularity,
            granularity.to_string(),
        )
    }

    pub fn set_enable_miscue(&mut self, enable_miscue: bool) -> Result<()> {
        self.set_property(
            PropertyId::PronunciationAssessmentEnableMiscue,
            enable_miscue.to_string(),
        )
    }

    pub fn set_phoneme_alphabet(&mut self, phoneme_alphabet: String) -> Result<()> {
        self.set_property(
            PropertyId::PronunciationAssessmentPhonemeAlphabet,
            phoneme_alphabet,
        )
    }

    pub fn set_nbest_phoneme_count(&mut self, nbest_phoneme_count: String) -> Result<()> {
        self.set_property(
            PropertyId::PronunciationAssessmentNBestPhonemeCount,
            nbest_phoneme_count,
        )
    }

    pub fn set_enable_prosody_assessment(&mut self, enable_prosody_assessment: bool) -> Result<()> {
        self.set_property(
            PropertyId::PronunciationAssessmentEnableProsodyAssessment,
            enable_prosody_assessment.to_string(),
        )
    }

    pub fn set_content_topic(&mut self, content_topic: String) -> Result<()> {
        self.set_property(
            PropertyId::PronunciationAssessmentContentTopic,
            content_topic,
        )
    }
}
