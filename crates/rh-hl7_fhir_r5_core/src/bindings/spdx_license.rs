use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/spdx-license
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpdxLicense {
    /// Not open source
    #[serde(rename = "not-open-source")]
    NotOpenSource,
    /// BSD Zero Clause License
    #[serde(rename = "0BSD")]
    Code0BSD,
    /// Attribution Assurance License
    #[serde(rename = "AAL")]
    AAL,
    /// Abstyles License
    #[serde(rename = "Abstyles")]
    Abstyles,
    /// Adobe Systems Incorporated Source Code License Agreement
    #[serde(rename = "Adobe-2006")]
    Adobe2006,
    /// Adobe Glyph List License
    #[serde(rename = "Adobe-Glyph")]
    AdobeGlyph,
    /// Amazon Digital Services License
    #[serde(rename = "ADSL")]
    ADSL,
    /// Academic Free License v1.1
    #[serde(rename = "AFL-1.1")]
    AFL11,
    /// Academic Free License v1.2
    #[serde(rename = "AFL-1.2")]
    AFL12,
    /// Academic Free License v2.0
    #[serde(rename = "AFL-2.0")]
    AFL20,
    /// Academic Free License v2.1
    #[serde(rename = "AFL-2.1")]
    AFL21,
    /// Academic Free License v3.0
    #[serde(rename = "AFL-3.0")]
    AFL30,
    /// Afmparse License
    #[serde(rename = "Afmparse")]
    Afmparse,
    /// Affero General Public License v1.0 only
    #[serde(rename = "AGPL-1.0-only")]
    AGPL10Only,
    /// Affero General Public License v1.0 or later
    #[serde(rename = "AGPL-1.0-or-later")]
    AGPL10OrLater,
    /// GNU Affero General Public License v3.0 only
    #[serde(rename = "AGPL-3.0-only")]
    AGPL30Only,
    /// GNU Affero General Public License v3.0 or later
    #[serde(rename = "AGPL-3.0-or-later")]
    AGPL30OrLater,
    /// Aladdin Free Public License
    #[serde(rename = "Aladdin")]
    Aladdin,
    /// AMD's plpa_map.c License
    #[serde(rename = "AMDPLPA")]
    AMDPLPA,
    /// Apple MIT License
    #[serde(rename = "AML")]
    AML,
    /// Academy of Motion Picture Arts and Sciences BSD
    #[serde(rename = "AMPAS")]
    AMPAS,
    /// ANTLR Software Rights Notice
    #[serde(rename = "ANTLR-PD")]
    ANTLRPD,
    /// Apache License 1.0
    #[serde(rename = "Apache-1.0")]
    Apache10,
    /// Apache License 1.1
    #[serde(rename = "Apache-1.1")]
    Apache11,
    /// Apache License 2.0
    #[serde(rename = "Apache-2.0")]
    Apache20,
    /// Adobe Postscript AFM License
    #[serde(rename = "APAFML")]
    APAFML,
    /// Adaptive Public License 1.0
    #[serde(rename = "APL-1.0")]
    APL10,
    /// Apple Public Source License 1.0
    #[serde(rename = "APSL-1.0")]
    APSL10,
    /// Apple Public Source License 1.1
    #[serde(rename = "APSL-1.1")]
    APSL11,
    /// Apple Public Source License 1.2
    #[serde(rename = "APSL-1.2")]
    APSL12,
    /// Apple Public Source License 2.0
    #[serde(rename = "APSL-2.0")]
    APSL20,
    /// Artistic License 1.0 w/clause 8
    #[serde(rename = "Artistic-1.0-cl8")]
    Artistic10Cl8,
    /// Artistic License 1.0 (Perl)
    #[serde(rename = "Artistic-1.0-Perl")]
    Artistic10Perl,
    /// Artistic License 1.0
    #[serde(rename = "Artistic-1.0")]
    Artistic10,
    /// Artistic License 2.0
    #[serde(rename = "Artistic-2.0")]
    Artistic20,
    /// Bahyph License
    #[serde(rename = "Bahyph")]
    Bahyph,
    /// Barr License
    #[serde(rename = "Barr")]
    Barr,
    /// Beerware License
    #[serde(rename = "Beerware")]
    Beerware,
    /// BitTorrent Open Source License v1.0
    #[serde(rename = "BitTorrent-1.0")]
    BitTorrent10,
    /// BitTorrent Open Source License v1.1
    #[serde(rename = "BitTorrent-1.1")]
    BitTorrent11,
    /// Borceux license
    #[serde(rename = "Borceux")]
    Borceux,
    /// BSD 1-Clause License
    #[serde(rename = "BSD-1-Clause")]
    BSD1Clause,
    /// BSD 2-Clause FreeBSD License
    #[serde(rename = "BSD-2-Clause-FreeBSD")]
    BSD2ClauseFreeBSD,
    /// BSD 2-Clause NetBSD License
    #[serde(rename = "BSD-2-Clause-NetBSD")]
    BSD2ClauseNetBSD,
    /// BSD-2-Clause Plus Patent License
    #[serde(rename = "BSD-2-Clause-Patent")]
    BSD2ClausePatent,
    /// BSD 2-Clause "Simplified" License
    #[serde(rename = "BSD-2-Clause")]
    BSD2Clause,
    /// BSD with attribution
    #[serde(rename = "BSD-3-Clause-Attribution")]
    BSD3ClauseAttribution,
    /// BSD 3-Clause Clear License
    #[serde(rename = "BSD-3-Clause-Clear")]
    BSD3ClauseClear,
    /// Lawrence Berkeley National Labs BSD variant license
    #[serde(rename = "BSD-3-Clause-LBNL")]
    BSD3ClauseLBNL,
    /// BSD 3-Clause No Nuclear License 2014
    #[serde(rename = "BSD-3-Clause-No-Nuclear-License-2014")]
    BSD3ClauseNoNuclearLicense2014,
    /// BSD 3-Clause No Nuclear License
    #[serde(rename = "BSD-3-Clause-No-Nuclear-License")]
    BSD3ClauseNoNuclearLicense,
    /// BSD 3-Clause No Nuclear Warranty
    #[serde(rename = "BSD-3-Clause-No-Nuclear-Warranty")]
    BSD3ClauseNoNuclearWarranty,
    /// BSD 3-Clause "New" or "Revised" License
    #[serde(rename = "BSD-3-Clause")]
    BSD3Clause,
    /// BSD-4-Clause (University of California-Specific)
    #[serde(rename = "BSD-4-Clause-UC")]
    BSD4ClauseUC,
    /// BSD 4-Clause "Original" or "Old" License
    #[serde(rename = "BSD-4-Clause")]
    BSD4Clause,
    /// BSD Protection License
    #[serde(rename = "BSD-Protection")]
    BSDProtection,
    /// BSD Source Code Attribution
    #[serde(rename = "BSD-Source-Code")]
    BSDSourceCode,
    /// Boost Software License 1.0
    #[serde(rename = "BSL-1.0")]
    BSL10,
    /// bzip2 and libbzip2 License v1.0.5
    #[serde(rename = "bzip2-1.0.5")]
    Bzip2105,
    /// bzip2 and libbzip2 License v1.0.6
    #[serde(rename = "bzip2-1.0.6")]
    Bzip2106,
    /// Caldera License
    #[serde(rename = "Caldera")]
    Caldera,
    /// Computer Associates Trusted Open Source License 1.1
    #[serde(rename = "CATOSL-1.1")]
    CATOSL11,
    /// Creative Commons Attribution 1.0 Generic
    #[serde(rename = "CC-BY-1.0")]
    CCBY10,
    /// Creative Commons Attribution 2.0 Generic
    #[serde(rename = "CC-BY-2.0")]
    CCBY20,
    /// Creative Commons Attribution 2.5 Generic
    #[serde(rename = "CC-BY-2.5")]
    CCBY25,
    /// Creative Commons Attribution 3.0 Unported
    #[serde(rename = "CC-BY-3.0")]
    CCBY30,
    /// Creative Commons Attribution 4.0 International
    #[serde(rename = "CC-BY-4.0")]
    CCBY40,
    /// Creative Commons Attribution Non Commercial 1.0 Generic
    #[serde(rename = "CC-BY-NC-1.0")]
    CCBYNC10,
    /// Creative Commons Attribution Non Commercial 2.0 Generic
    #[serde(rename = "CC-BY-NC-2.0")]
    CCBYNC20,
    /// Creative Commons Attribution Non Commercial 2.5 Generic
    #[serde(rename = "CC-BY-NC-2.5")]
    CCBYNC25,
    /// Creative Commons Attribution Non Commercial 3.0 Unported
    #[serde(rename = "CC-BY-NC-3.0")]
    CCBYNC30,
    /// Creative Commons Attribution Non Commercial 4.0 International
    #[serde(rename = "CC-BY-NC-4.0")]
    CCBYNC40,
    /// Creative Commons Attribution Non Commercial No Derivatives 1.0 Generic
    #[serde(rename = "CC-BY-NC-ND-1.0")]
    CCBYNCND10,
    /// Creative Commons Attribution Non Commercial No Derivatives 2.0 Generic
    #[serde(rename = "CC-BY-NC-ND-2.0")]
    CCBYNCND20,
    /// Creative Commons Attribution Non Commercial No Derivatives 2.5 Generic
    #[serde(rename = "CC-BY-NC-ND-2.5")]
    CCBYNCND25,
    /// Creative Commons Attribution Non Commercial No Derivatives 3.0 Unported
    #[serde(rename = "CC-BY-NC-ND-3.0")]
    CCBYNCND30,
    /// Creative Commons Attribution Non Commercial No Derivatives 4.0 International
    #[serde(rename = "CC-BY-NC-ND-4.0")]
    CCBYNCND40,
    /// Creative Commons Attribution Non Commercial Share Alike 1.0 Generic
    #[serde(rename = "CC-BY-NC-SA-1.0")]
    CCBYNCSA10,
    /// Creative Commons Attribution Non Commercial Share Alike 2.0 Generic
    #[serde(rename = "CC-BY-NC-SA-2.0")]
    CCBYNCSA20,
    /// Creative Commons Attribution Non Commercial Share Alike 2.5 Generic
    #[serde(rename = "CC-BY-NC-SA-2.5")]
    CCBYNCSA25,
    /// Creative Commons Attribution Non Commercial Share Alike 3.0 Unported
    #[serde(rename = "CC-BY-NC-SA-3.0")]
    CCBYNCSA30,
    /// Creative Commons Attribution Non Commercial Share Alike 4.0 International
    #[serde(rename = "CC-BY-NC-SA-4.0")]
    CCBYNCSA40,
    /// Creative Commons Attribution No Derivatives 1.0 Generic
    #[serde(rename = "CC-BY-ND-1.0")]
    CCBYND10,
    /// Creative Commons Attribution No Derivatives 2.0 Generic
    #[serde(rename = "CC-BY-ND-2.0")]
    CCBYND20,
    /// Creative Commons Attribution No Derivatives 2.5 Generic
    #[serde(rename = "CC-BY-ND-2.5")]
    CCBYND25,
    /// Creative Commons Attribution No Derivatives 3.0 Unported
    #[serde(rename = "CC-BY-ND-3.0")]
    CCBYND30,
    /// Creative Commons Attribution No Derivatives 4.0 International
    #[serde(rename = "CC-BY-ND-4.0")]
    CCBYND40,
    /// Creative Commons Attribution Share Alike 1.0 Generic
    #[serde(rename = "CC-BY-SA-1.0")]
    CCBYSA10,
    /// Creative Commons Attribution Share Alike 2.0 Generic
    #[serde(rename = "CC-BY-SA-2.0")]
    CCBYSA20,
    /// Creative Commons Attribution Share Alike 2.5 Generic
    #[serde(rename = "CC-BY-SA-2.5")]
    CCBYSA25,
    /// Creative Commons Attribution Share Alike 3.0 Unported
    #[serde(rename = "CC-BY-SA-3.0")]
    CCBYSA30,
    /// Creative Commons Attribution Share Alike 4.0 International
    #[serde(rename = "CC-BY-SA-4.0")]
    CCBYSA40,
    /// Creative Commons Zero v1.0 Universal
    #[serde(rename = "CC0-1.0")]
    CC010,
    /// Common Development and Distribution License 1.0
    #[serde(rename = "CDDL-1.0")]
    CDDL10,
    /// Common Development and Distribution License 1.1
    #[serde(rename = "CDDL-1.1")]
    CDDL11,
    /// Community Data License Agreement Permissive 1.0
    #[serde(rename = "CDLA-Permissive-1.0")]
    CDLAPermissive10,
    /// Community Data License Agreement Sharing 1.0
    #[serde(rename = "CDLA-Sharing-1.0")]
    CDLASharing10,
    /// CeCILL Free Software License Agreement v1.0
    #[serde(rename = "CECILL-1.0")]
    CECILL10,
    /// CeCILL Free Software License Agreement v1.1
    #[serde(rename = "CECILL-1.1")]
    CECILL11,
    /// CeCILL Free Software License Agreement v2.0
    #[serde(rename = "CECILL-2.0")]
    CECILL20,
    /// CeCILL Free Software License Agreement v2.1
    #[serde(rename = "CECILL-2.1")]
    CECILL21,
    /// CeCILL-B Free Software License Agreement
    #[serde(rename = "CECILL-B")]
    CECILLB,
    /// CeCILL-C Free Software License Agreement
    #[serde(rename = "CECILL-C")]
    CECILLC,
    /// Clarified Artistic License
    #[serde(rename = "ClArtistic")]
    ClArtistic,
    /// CNRI Jython License
    #[serde(rename = "CNRI-Jython")]
    CNRIJython,
    /// CNRI Python Open Source GPL Compatible License Agreement
    #[serde(rename = "CNRI-Python-GPL-Compatible")]
    CNRIPythonGPLCompatible,
    /// CNRI Python License
    #[serde(rename = "CNRI-Python")]
    CNRIPython,
    /// Condor Public License v1.1
    #[serde(rename = "Condor-1.1")]
    Condor11,
    /// Common Public Attribution License 1.0
    #[serde(rename = "CPAL-1.0")]
    CPAL10,
    /// Common Public License 1.0
    #[serde(rename = "CPL-1.0")]
    CPL10,
    /// Code Project Open License 1.02
    #[serde(rename = "CPOL-1.02")]
    CPOL102,
    /// Crossword License
    #[serde(rename = "Crossword")]
    Crossword,
    /// CrystalStacker License
    #[serde(rename = "CrystalStacker")]
    CrystalStacker,
    /// CUA Office Public License v1.0
    #[serde(rename = "CUA-OPL-1.0")]
    CUAOPL10,
    /// Cube License
    #[serde(rename = "Cube")]
    Cube,
    /// curl License
    #[serde(rename = "curl")]
    Curl,
    /// Deutsche Freie Software Lizenz
    #[serde(rename = "D-FSL-1.0")]
    DFSL10,
    /// diffmark license
    #[serde(rename = "diffmark")]
    Diffmark,
    /// DOC License
    #[serde(rename = "DOC")]
    DOC,
    /// Dotseqn License
    #[serde(rename = "Dotseqn")]
    Dotseqn,
    /// DSDP License
    #[serde(rename = "DSDP")]
    DSDP,
    /// dvipdfm License
    #[serde(rename = "dvipdfm")]
    Dvipdfm,
    /// Educational Community License v1.0
    #[serde(rename = "ECL-1.0")]
    ECL10,
    /// Educational Community License v2.0
    #[serde(rename = "ECL-2.0")]
    ECL20,
    /// Eiffel Forum License v1.0
    #[serde(rename = "EFL-1.0")]
    EFL10,
    /// Eiffel Forum License v2.0
    #[serde(rename = "EFL-2.0")]
    EFL20,
    /// eGenix.com Public License 1.1.0
    #[serde(rename = "eGenix")]
    EGenix,
    /// Entessa Public License v1.0
    #[serde(rename = "Entessa")]
    Entessa,
    /// Eclipse Public License 1.0
    #[serde(rename = "EPL-1.0")]
    EPL10,
    /// Eclipse Public License 2.0
    #[serde(rename = "EPL-2.0")]
    EPL20,
    /// Erlang Public License v1.1
    #[serde(rename = "ErlPL-1.1")]
    ErlPL11,
    /// EU DataGrid Software License
    #[serde(rename = "EUDatagrid")]
    EUDatagrid,
    /// European Union Public License 1.0
    #[serde(rename = "EUPL-1.0")]
    EUPL10,
    /// European Union Public License 1.1
    #[serde(rename = "EUPL-1.1")]
    EUPL11,
    /// European Union Public License 1.2
    #[serde(rename = "EUPL-1.2")]
    EUPL12,
    /// Eurosym License
    #[serde(rename = "Eurosym")]
    Eurosym,
    /// Fair License
    #[serde(rename = "Fair")]
    Fair,
    /// Frameworx Open License 1.0
    #[serde(rename = "Frameworx-1.0")]
    Frameworx10,
    /// FreeImage Public License v1.0
    #[serde(rename = "FreeImage")]
    FreeImage,
    /// FSF All Permissive License
    #[serde(rename = "FSFAP")]
    FSFAP,
    /// FSF Unlimited License
    #[serde(rename = "FSFUL")]
    FSFUL,
    /// FSF Unlimited License (with License Retention)
    #[serde(rename = "FSFULLR")]
    FSFULLR,
    /// Freetype Project License
    #[serde(rename = "FTL")]
    FTL,
    /// GNU Free Documentation License v1.1 only
    #[serde(rename = "GFDL-1.1-only")]
    GFDL11Only,
    /// GNU Free Documentation License v1.1 or later
    #[serde(rename = "GFDL-1.1-or-later")]
    GFDL11OrLater,
    /// GNU Free Documentation License v1.2 only
    #[serde(rename = "GFDL-1.2-only")]
    GFDL12Only,
    /// GNU Free Documentation License v1.2 or later
    #[serde(rename = "GFDL-1.2-or-later")]
    GFDL12OrLater,
    /// GNU Free Documentation License v1.3 only
    #[serde(rename = "GFDL-1.3-only")]
    GFDL13Only,
    /// GNU Free Documentation License v1.3 or later
    #[serde(rename = "GFDL-1.3-or-later")]
    GFDL13OrLater,
    /// Giftware License
    #[serde(rename = "Giftware")]
    Giftware,
    /// GL2PS License
    #[serde(rename = "GL2PS")]
    GL2PS,
    /// 3dfx Glide License
    #[serde(rename = "Glide")]
    Glide,
    /// Glulxe License
    #[serde(rename = "Glulxe")]
    Glulxe,
    /// gnuplot License
    #[serde(rename = "gnuplot")]
    Gnuplot,
    /// GNU General Public License v1.0 only
    #[serde(rename = "GPL-1.0-only")]
    GPL10Only,
    /// GNU General Public License v1.0 or later
    #[serde(rename = "GPL-1.0-or-later")]
    GPL10OrLater,
    /// GNU General Public License v2.0 only
    #[serde(rename = "GPL-2.0-only")]
    GPL20Only,
    /// GNU General Public License v2.0 or later
    #[serde(rename = "GPL-2.0-or-later")]
    GPL20OrLater,
    /// GNU General Public License v3.0 only
    #[serde(rename = "GPL-3.0-only")]
    GPL30Only,
    /// GNU General Public License v3.0 or later
    #[serde(rename = "GPL-3.0-or-later")]
    GPL30OrLater,
    /// gSOAP Public License v1.3b
    #[serde(rename = "gSOAP-1.3b")]
    GSOAP13b,
    /// Haskell Language Report License
    #[serde(rename = "HaskellReport")]
    HaskellReport,
    /// Historical Permission Notice and Disclaimer
    #[serde(rename = "HPND")]
    HPND,
    /// IBM PowerPC Initialization and Boot Software
    #[serde(rename = "IBM-pibs")]
    IBMPibs,
    /// ICU License
    #[serde(rename = "ICU")]
    ICU,
    /// Independent JPEG Group License
    #[serde(rename = "IJG")]
    IJG,
    /// ImageMagick License
    #[serde(rename = "ImageMagick")]
    ImageMagick,
    /// iMatix Standard Function Library Agreement
    #[serde(rename = "iMatix")]
    IMatix,
    /// Imlib2 License
    #[serde(rename = "Imlib2")]
    Imlib2,
    /// Info-ZIP License
    #[serde(rename = "Info-ZIP")]
    InfoZIP,
    /// Intel ACPI Software License Agreement
    #[serde(rename = "Intel-ACPI")]
    IntelACPI,
    /// Intel Open Source License
    #[serde(rename = "Intel")]
    Intel,
    /// Interbase Public License v1.0
    #[serde(rename = "Interbase-1.0")]
    Interbase10,
    /// IPA Font License
    #[serde(rename = "IPA")]
    IPA,
    /// IBM Public License v1.0
    #[serde(rename = "IPL-1.0")]
    IPL10,
    /// ISC License
    #[serde(rename = "ISC")]
    ISC,
    /// JasPer License
    #[serde(rename = "JasPer-2.0")]
    JasPer20,
    /// JSON License
    #[serde(rename = "JSON")]
    JSON,
    /// Licence Art Libre 1.2
    #[serde(rename = "LAL-1.2")]
    LAL12,
    /// Licence Art Libre 1.3
    #[serde(rename = "LAL-1.3")]
    LAL13,
    /// Latex2e License
    #[serde(rename = "Latex2e")]
    Latex2e,
    /// Leptonica License
    #[serde(rename = "Leptonica")]
    Leptonica,
    /// GNU Library General Public License v2 only
    #[serde(rename = "LGPL-2.0-only")]
    LGPL20Only,
    /// GNU Library General Public License v2 or later
    #[serde(rename = "LGPL-2.0-or-later")]
    LGPL20OrLater,
    /// GNU Lesser General Public License v2.1 only
    #[serde(rename = "LGPL-2.1-only")]
    LGPL21Only,
    /// GNU Lesser General Public License v2.1 or later
    #[serde(rename = "LGPL-2.1-or-later")]
    LGPL21OrLater,
    /// GNU Lesser General Public License v3.0 only
    #[serde(rename = "LGPL-3.0-only")]
    LGPL30Only,
    /// GNU Lesser General Public License v3.0 or later
    #[serde(rename = "LGPL-3.0-or-later")]
    LGPL30OrLater,
    /// Lesser General Public License For Linguistic Resources
    #[serde(rename = "LGPLLR")]
    LGPLLR,
    /// libpng License
    #[serde(rename = "Libpng")]
    Libpng,
    /// libtiff License
    #[serde(rename = "libtiff")]
    Libtiff,
    /// Licence Libre du Québec – Permissive version 1.1
    #[serde(rename = "LiLiQ-P-1.1")]
    LiLiQP11,
    /// Licence Libre du Québec – Réciprocité version 1.1
    #[serde(rename = "LiLiQ-R-1.1")]
    LiLiQR11,
    /// Licence Libre du Québec – Réciprocité forte version 1.1
    #[serde(rename = "LiLiQ-Rplus-1.1")]
    LiLiQRplus11,
    /// Linux Kernel Variant of OpenIB.org license
    #[serde(rename = "Linux-OpenIB")]
    LinuxOpenIB,
    /// Lucent Public License Version 1.0
    #[serde(rename = "LPL-1.0")]
    LPL10,
    /// Lucent Public License v1.02
    #[serde(rename = "LPL-1.02")]
    LPL102,
    /// LaTeX Project Public License v1.0
    #[serde(rename = "LPPL-1.0")]
    LPPL10,
    /// LaTeX Project Public License v1.1
    #[serde(rename = "LPPL-1.1")]
    LPPL11,
    /// LaTeX Project Public License v1.2
    #[serde(rename = "LPPL-1.2")]
    LPPL12,
    /// LaTeX Project Public License v1.3a
    #[serde(rename = "LPPL-1.3a")]
    LPPL13a,
    /// LaTeX Project Public License v1.3c
    #[serde(rename = "LPPL-1.3c")]
    LPPL13c,
    /// MakeIndex License
    #[serde(rename = "MakeIndex")]
    MakeIndex,
    /// MirOS License
    #[serde(rename = "MirOS")]
    MirOS,
    /// MIT No Attribution
    #[serde(rename = "MIT-0")]
    MIT0,
    /// Enlightenment License (e16)
    #[serde(rename = "MIT-advertising")]
    MITAdvertising,
    /// CMU License
    #[serde(rename = "MIT-CMU")]
    MITCMU,
    /// enna License
    #[serde(rename = "MIT-enna")]
    MITEnna,
    /// feh License
    #[serde(rename = "MIT-feh")]
    MITFeh,
    /// MIT License
    #[serde(rename = "MIT")]
    MIT,
    /// MIT +no-false-attribs license
    #[serde(rename = "MITNFA")]
    MITNFA,
    /// Motosoto License
    #[serde(rename = "Motosoto")]
    Motosoto,
    /// mpich2 License
    #[serde(rename = "mpich2")]
    Mpich2,
    /// Mozilla Public License 1.0
    #[serde(rename = "MPL-1.0")]
    MPL10,
    /// Mozilla Public License 1.1
    #[serde(rename = "MPL-1.1")]
    MPL11,
    /// Mozilla Public License 2.0 (no copyleft exception)
    #[serde(rename = "MPL-2.0-no-copyleft-exception")]
    MPL20NoCopyleftException,
    /// Mozilla Public License 2.0
    #[serde(rename = "MPL-2.0")]
    MPL20,
    /// Microsoft Public License
    #[serde(rename = "MS-PL")]
    MSPL,
    /// Microsoft Reciprocal License
    #[serde(rename = "MS-RL")]
    MSRL,
    /// Matrix Template Library License
    #[serde(rename = "MTLL")]
    MTLL,
    /// Multics License
    #[serde(rename = "Multics")]
    Multics,
    /// Mup License
    #[serde(rename = "Mup")]
    Mup,
    /// NASA Open Source Agreement 1.3
    #[serde(rename = "NASA-1.3")]
    NASA13,
    /// Naumen Public License
    #[serde(rename = "Naumen")]
    Naumen,
    /// Net Boolean Public License v1
    #[serde(rename = "NBPL-1.0")]
    NBPL10,
    /// University of Illinois/NCSA Open Source License
    #[serde(rename = "NCSA")]
    NCSA,
    /// Net-SNMP License
    #[serde(rename = "Net-SNMP")]
    NetSNMP,
    /// NetCDF license
    #[serde(rename = "NetCDF")]
    NetCDF,
    /// Newsletr License
    #[serde(rename = "Newsletr")]
    Newsletr,
    /// Nethack General Public License
    #[serde(rename = "NGPL")]
    NGPL,
    /// Norwegian Licence for Open Government Data
    #[serde(rename = "NLOD-1.0")]
    NLOD10,
    /// No Limit Public License
    #[serde(rename = "NLPL")]
    NLPL,
    /// Nokia Open Source License
    #[serde(rename = "Nokia")]
    Nokia,
    /// Netizen Open Source License
    #[serde(rename = "NOSL")]
    NOSL,
    /// Noweb License
    #[serde(rename = "Noweb")]
    Noweb,
    /// Netscape Public License v1.0
    #[serde(rename = "NPL-1.0")]
    NPL10,
    /// Netscape Public License v1.1
    #[serde(rename = "NPL-1.1")]
    NPL11,
    /// Non-Profit Open Software License 3.0
    #[serde(rename = "NPOSL-3.0")]
    NPOSL30,
    /// NRL License
    #[serde(rename = "NRL")]
    NRL,
    /// NTP License
    #[serde(rename = "NTP")]
    NTP,
    /// Open CASCADE Technology Public License
    #[serde(rename = "OCCT-PL")]
    OCCTPL,
    /// OCLC Research Public License 2.0
    #[serde(rename = "OCLC-2.0")]
    OCLC20,
    /// ODC Open Database License v1.0
    #[serde(rename = "ODbL-1.0")]
    ODbL10,
    /// SIL Open Font License 1.0
    #[serde(rename = "OFL-1.0")]
    OFL10,
    /// SIL Open Font License 1.1
    #[serde(rename = "OFL-1.1")]
    OFL11,
    /// Open Group Test Suite License
    #[serde(rename = "OGTSL")]
    OGTSL,
    /// Open LDAP Public License v1.1
    #[serde(rename = "OLDAP-1.1")]
    OLDAP11,
    /// Open LDAP Public License v1.2
    #[serde(rename = "OLDAP-1.2")]
    OLDAP12,
    /// Open LDAP Public License v1.3
    #[serde(rename = "OLDAP-1.3")]
    OLDAP13,
    /// Open LDAP Public License v1.4
    #[serde(rename = "OLDAP-1.4")]
    OLDAP14,
    /// Open LDAP Public License v2.0.1
    #[serde(rename = "OLDAP-2.0.1")]
    OLDAP201,
    /// Open LDAP Public License v2.0 (or possibly 2.0A and 2.0B)
    #[serde(rename = "OLDAP-2.0")]
    OLDAP20,
    /// Open LDAP Public License v2.1
    #[serde(rename = "OLDAP-2.1")]
    OLDAP21,
    /// Open LDAP Public License v2.2.1
    #[serde(rename = "OLDAP-2.2.1")]
    OLDAP221,
    /// Open LDAP Public License 2.2.2
    #[serde(rename = "OLDAP-2.2.2")]
    OLDAP222,
    /// Open LDAP Public License v2.2
    #[serde(rename = "OLDAP-2.2")]
    OLDAP22,
    /// Open LDAP Public License v2.3
    #[serde(rename = "OLDAP-2.3")]
    OLDAP23,
    /// Open LDAP Public License v2.4
    #[serde(rename = "OLDAP-2.4")]
    OLDAP24,
    /// Open LDAP Public License v2.5
    #[serde(rename = "OLDAP-2.5")]
    OLDAP25,
    /// Open LDAP Public License v2.6
    #[serde(rename = "OLDAP-2.6")]
    OLDAP26,
    /// Open LDAP Public License v2.7
    #[serde(rename = "OLDAP-2.7")]
    OLDAP27,
    /// Open LDAP Public License v2.8
    #[serde(rename = "OLDAP-2.8")]
    OLDAP28,
    /// Open Market License
    #[serde(rename = "OML")]
    OML,
    /// OpenSSL License
    #[serde(rename = "OpenSSL")]
    OpenSSL,
    /// Open Public License v1.0
    #[serde(rename = "OPL-1.0")]
    OPL10,
    /// OSET Public License version 2.1
    #[serde(rename = "OSET-PL-2.1")]
    OSETPL21,
    /// Open Software License 1.0
    #[serde(rename = "OSL-1.0")]
    OSL10,
    /// Open Software License 1.1
    #[serde(rename = "OSL-1.1")]
    OSL11,
    /// Open Software License 2.0
    #[serde(rename = "OSL-2.0")]
    OSL20,
    /// Open Software License 2.1
    #[serde(rename = "OSL-2.1")]
    OSL21,
    /// Open Software License 3.0
    #[serde(rename = "OSL-3.0")]
    OSL30,
    /// ODC Public Domain Dedication & License 1.0
    #[serde(rename = "PDDL-1.0")]
    PDDL10,
    /// PHP License v3.0
    #[serde(rename = "PHP-3.0")]
    PHP30,
    /// PHP License v3.01
    #[serde(rename = "PHP-3.01")]
    PHP301,
    /// Plexus Classworlds License
    #[serde(rename = "Plexus")]
    Plexus,
    /// PostgreSQL License
    #[serde(rename = "PostgreSQL")]
    PostgreSQL,
    /// psfrag License
    #[serde(rename = "psfrag")]
    Psfrag,
    /// psutils License
    #[serde(rename = "psutils")]
    Psutils,
    /// Python License 2.0
    #[serde(rename = "Python-2.0")]
    Python20,
    /// Qhull License
    #[serde(rename = "Qhull")]
    Qhull,
    /// Q Public License 1.0
    #[serde(rename = "QPL-1.0")]
    QPL10,
    /// Rdisc License
    #[serde(rename = "Rdisc")]
    Rdisc,
    /// Red Hat eCos Public License v1.1
    #[serde(rename = "RHeCos-1.1")]
    RHeCos11,
    /// Reciprocal Public License 1.1
    #[serde(rename = "RPL-1.1")]
    RPL11,
    /// Reciprocal Public License 1.5
    #[serde(rename = "RPL-1.5")]
    RPL15,
    /// RealNetworks Public Source License v1.0
    #[serde(rename = "RPSL-1.0")]
    RPSL10,
    /// RSA Message-Digest License
    #[serde(rename = "RSA-MD")]
    RSAMD,
    /// Ricoh Source Code Public License
    #[serde(rename = "RSCPL")]
    RSCPL,
    /// Ruby License
    #[serde(rename = "Ruby")]
    Ruby,
    /// Sax Public Domain Notice
    #[serde(rename = "SAX-PD")]
    SAXPD,
    /// Saxpath License
    #[serde(rename = "Saxpath")]
    Saxpath,
    /// SCEA Shared Source License
    #[serde(rename = "SCEA")]
    SCEA,
    /// Sendmail License
    #[serde(rename = "Sendmail")]
    Sendmail,
    /// SGI Free Software License B v1.0
    #[serde(rename = "SGI-B-1.0")]
    SGIB10,
    /// SGI Free Software License B v1.1
    #[serde(rename = "SGI-B-1.1")]
    SGIB11,
    /// SGI Free Software License B v2.0
    #[serde(rename = "SGI-B-2.0")]
    SGIB20,
    /// Simple Public License 2.0
    #[serde(rename = "SimPL-2.0")]
    SimPL20,
    /// Sun Industry Standards Source License v1.2
    #[serde(rename = "SISSL-1.2")]
    SISSL12,
    /// Sun Industry Standards Source License v1.1
    #[serde(rename = "SISSL")]
    SISSL,
    /// Sleepycat License
    #[serde(rename = "Sleepycat")]
    Sleepycat,
    /// Standard ML of New Jersey License
    #[serde(rename = "SMLNJ")]
    SMLNJ,
    /// Secure Messaging Protocol Public License
    #[serde(rename = "SMPPL")]
    SMPPL,
    /// SNIA Public License 1.1
    #[serde(rename = "SNIA")]
    SNIA,
    /// Spencer License 86
    #[serde(rename = "Spencer-86")]
    Spencer86,
    /// Spencer License 94
    #[serde(rename = "Spencer-94")]
    Spencer94,
    /// Spencer License 99
    #[serde(rename = "Spencer-99")]
    Spencer99,
    /// Sun Public License v1.0
    #[serde(rename = "SPL-1.0")]
    SPL10,
    /// SugarCRM Public License v1.1.3
    #[serde(rename = "SugarCRM-1.1.3")]
    SugarCRM113,
    /// Scheme Widget Library (SWL) Software License Agreement
    #[serde(rename = "SWL")]
    SWL,
    /// TCL/TK License
    #[serde(rename = "TCL")]
    TCL,
    /// TCP Wrappers License
    #[serde(rename = "TCP-wrappers")]
    TCPWrappers,
    /// TMate Open Source License
    #[serde(rename = "TMate")]
    TMate,
    /// TORQUE v2.5+ Software License v1.1
    #[serde(rename = "TORQUE-1.1")]
    TORQUE11,
    /// Trusster Open Source License
    #[serde(rename = "TOSL")]
    TOSL,
    /// Unicode License Agreement - Data Files and Software (2015)
    #[serde(rename = "Unicode-DFS-2015")]
    UnicodeDFS2015,
    /// Unicode License Agreement - Data Files and Software (2016)
    #[serde(rename = "Unicode-DFS-2016")]
    UnicodeDFS2016,
    /// Unicode Terms of Use
    #[serde(rename = "Unicode-TOU")]
    UnicodeTOU,
    /// The Unlicense
    #[serde(rename = "Unlicense")]
    Unlicense,
    /// Universal Permissive License v1.0
    #[serde(rename = "UPL-1.0")]
    UPL10,
    /// Vim License
    #[serde(rename = "Vim")]
    Vim,
    /// VOSTROM Public License for Open Source
    #[serde(rename = "VOSTROM")]
    VOSTROM,
    /// Vovida Software License v1.0
    #[serde(rename = "VSL-1.0")]
    VSL10,
    /// W3C Software Notice and License (1998-07-20)
    #[serde(rename = "W3C-19980720")]
    W3C19980720,
    /// W3C Software Notice and Document License (2015-05-13)
    #[serde(rename = "W3C-20150513")]
    W3C20150513,
    /// W3C Software Notice and License (2002-12-31)
    #[serde(rename = "W3C")]
    W3C,
    /// Sybase Open Watcom Public License 1.0
    #[serde(rename = "Watcom-1.0")]
    Watcom10,
    /// Wsuipa License
    #[serde(rename = "Wsuipa")]
    Wsuipa,
    /// Do What The F*ck You Want To Public License
    #[serde(rename = "WTFPL")]
    WTFPL,
    /// X11 License
    #[serde(rename = "X11")]
    X11,
    /// Xerox License
    #[serde(rename = "Xerox")]
    Xerox,
    /// XFree86 License 1.1
    #[serde(rename = "XFree86-1.1")]
    XFree8611,
    /// xinetd License
    #[serde(rename = "xinetd")]
    Xinetd,
    /// X.Net License
    #[serde(rename = "Xnet")]
    Xnet,
    /// XPP License
    #[serde(rename = "xpp")]
    Xpp,
    /// XSkat License
    #[serde(rename = "XSkat")]
    XSkat,
    /// Yahoo! Public License v1.0
    #[serde(rename = "YPL-1.0")]
    YPL10,
    /// Yahoo! Public License v1.1
    #[serde(rename = "YPL-1.1")]
    YPL11,
    /// Zed License
    #[serde(rename = "Zed")]
    Zed,
    /// Zend License v2.0
    #[serde(rename = "Zend-2.0")]
    Zend20,
    /// Zimbra Public License v1.3
    #[serde(rename = "Zimbra-1.3")]
    Zimbra13,
    /// Zimbra Public License v1.4
    #[serde(rename = "Zimbra-1.4")]
    Zimbra14,
    /// zlib/libpng License with Acknowledgement
    #[serde(rename = "zlib-acknowledgement")]
    ZlibAcknowledgement,
    /// zlib License
    #[serde(rename = "Zlib")]
    Zlib,
    /// Zope Public License 1.1
    #[serde(rename = "ZPL-1.1")]
    ZPL11,
    /// Zope Public License 2.0
    #[serde(rename = "ZPL-2.0")]
    ZPL20,
    /// Zope Public License 2.1
    #[serde(rename = "ZPL-2.1")]
    ZPL21,
}
impl Default for SpdxLicense {
    fn default() -> Self {
        Self::NotOpenSource
    }
}
