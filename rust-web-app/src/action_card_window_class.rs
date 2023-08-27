// Decompiled with JetBrains decompiler
#![allow(non_snake_case)]
// Type: WindowsApplication1.ActionCardWindowClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// using Microsoft.VisualBasic;
// using System;
// using System.Drawing;
// using System.IO;
// using System.Windows.Forms;

use std::fs::File;
use std::path::Path;
use crate::list_class::ListClass;

// namespace WindowsApplication1
// {
// pub class ActionCardWindowClass : WindowClass
#[derive(Clone, Debug, Default)]
pub struct ActionCardWindowClass {
    UnitListId: i32,
    UnitListObj: ListClass,
    libListId: i32,
    libListObj: ListClass,
    UnitList2Id: i32,
    UnitList2Obj: ListClass,
    BAddUnitId: i32,
    cloneid: i32,
    clonetextid: i32,
    BAddUnitTextId: i32,
    BNameId: i32,
    BNameTextId: i32,
    BRemoveUnitId: i32,
    BRemoveUnitTextId: i32,
    b1id: i32,
    b1bid: i32,
    b2id: i32,
    b2bid: i32,
    b3id: i32,
    b3bid: i32,
    b4id: i32,
    b4bid: i32,
    a1id: i32,
    a1bid: i32,
    a2id: i32,
    a2bid: i32,
    a3id: i32,
    a3bid: i32,
    a4id: i32,
    a4bid: i32,
    a5id: i32,
    a5bid: i32,
    aa4id: i32,
    aa4bid: i32,
    aa5id: i32,
    aa5bid: i32,
    a6id: i32,
    a6bid: i32,
    a7id: i32,
    a7bid: i32,
    a8id: i32,
    a8bid: i32,
    a9id: i32,
    a9bid: i32,
    a10id: i32,
    a10bid: i32,
    a11id: i32,
    a11bid: i32,
    a12id: i32,
    a12bid: i32,
    a13id: i32,
    a13bid: i32,
    aa13id: i32,
    aa13bid: i32,
    a14id: i32,
    a14bid: i32,
    a15id: i32,
    a15bid: i32,
    a16id: i32,
    a16bid: i32,
    a17id: i32,
    a17bid: i32,
    a18id: i32,
    a18bid: i32,
    a19id: i32,
    a19bid: i32,
    a20id: i32,
    a20bid: i32,
    a21id: i32,
    a21bid: i32,
    a22id: i32,
    a22bid: i32,
    a23id: i32,
    a23bid: i32,
    UnitNr: i32,
    ceilingnr: i32,
    x1id: i32,
    x2id: i32,
    x3id: i32,
    libnr: i32,
    ss: String,
}

impl ActionCardWindowClass {

    // TODO
    // pub ActionCardWindowClass( tGame: GameClass)
    //   : base( tGame, tGame.ScreenWidth, tGame.ScreenHeight - 100, tDoBorders: 1, tHeaderString: "Action Cards")
    // {
    //   self.UnitNr = -1;
    //   self.ceilingnr = -1;
    //   self.libnr = -1;
    //   self.MakeUnitListGUI();
    // }

    // TODO
    // pub fn Doresh() => self.MakeUnitTypeItemGUI();

    pub fn MakeUnitListGUI(mut self) {
        if self.UnitListId > 0 {
            self.RemoveSubPart(self.UnitListId);
        }
        if self.libListId > 0 {
            self.RemoveSubPart(self.libListId);
        }
        self.libListObj = ListClass::new();
        self.libListObj.add("All", -2);
        let mut num1: i32 = -1;
        let mut num2: i32 = 0;
        let mut libraryCounter: i32 = self.game.Data.LibraryCounter;
        // for (let mut index: i32 = 0; index <= libraryCounter; index += 1)
        let mut index: i32 = 0;
        while index <= libraryCounter {
            num2 += 1;
            if self.libnr == index {
                num1 = num2;
            }
            self.libListObj.add(Conversion.Str(index) + ") " + self.game.Data.LibraryObj[index].name, index);
            index += 1;
        }
        if self.libnr == -1 {
            num1 = 0;
        }
        let libListObj: ListClass = self.libListObj;
        let mut tlistselect1: i32 = num1;
        let mut game1: GameClass = self.game;
        let local1: Bitmap = self.OwnBitmap;
        let mut font1: Font;
        let mut local2 = &font1;
        // let mut tsubpart1: SubPartClass = ListSubPartClass::new(libListObj, 9, 200, tlistselect1, game1, tHeader: "Libraries", tbackbitmap: ( local1), bbx: 10, bby: 55, overruleFont: ( local2));
        let mut tsubpart1: SubPartClass = ListSubPartClass::new(libListObj, 9, 200, tlistselect1, game1, "Libraries", local1, 10, 55, local2);
        // self.libListId = self.AddSubPart( tsubpart1, 10, 55, 200, 192, 0);
        self.libListId = self.AddSubPart(tsubpart1, 10, 55, 200, 192, 0);
        let mut num3: i32 = -1;
        let mut num4: i32 = -1;
        if self.game.Data.ActionCardCounter > -1 {
            self.UnitListObj = ListClass::new();
            let mut actionCardCounter: i32 = self.game.Data.ActionCardCounter;
            let mut index: i32 = 0;
            // for (let mut index: i32 = 0; index <= actionCardCounter; index += 1)
            while index <= actionCardCounter {
                if (self.game.Data.ActionCardObj[index].LibId.libSlot == self.libnr) | (self.libnr == -1) {
                    num4 += 1;
                    self.UnitListObj.add(Strings.Trim(Conversion.Str(index)) + ") " + self.game.Data.ActionCardObj[index].Title, index);
                    if self.UnitNr == index {
                        num3 = num4;
                    }
                }
                index += 1;
            }
            let mut unitListObj: ListClass = self.UnitListObj;
            let mut tlistselect2: i32 = num3;
            let mut game2: GameClass = self.game;
            let mut local3: Bitmap = self.OwnBitmap;
            // let mut font2: Font =  null;
            let mut font2: Font;
            let mut local4 = &font2;
            // let mut tsubpart2: SubPartClass = ListSubPartClass::new(unitListObj, 20, 300, tlistselect2, game2, tHeader: "Action Cards", tbackbitmap: ( local3), bbx: 10, bby: 277, overruleFont: ( local4));
            let mut tsubpart2: SubPartClass = ListSubPartClass::new(unitListObj, 20, 300, tlistselect2, game2, "Action Cards", local3, 10, 277, local4);
            self.UnitListId = self.AddSubPart(tsubpart2, 10, 277, 300, 352, 0);
            if self.UnitNr > -1 {
                self.MakeUnitTypeItemGUI();
            }
        }
        if self.BAddUnitId > 0 {
            self.RemoveSubPart(self.BAddUnitId);
        }
        if self.BAddUnitTextId > 0 {
            self.RemoveSubPart(self.BAddUnitTextId);
        }
        if self.b2id > 0 {
            self.RemoveSubPart(self.b2id);
        }
        if self.b2bid > 0 {
            self.RemoveSubPart(self.b2bid);
        }
        self.ss = "Click to add an action card".to_string();
        // let mut tsubpart3: SubPartClass = ButtonPartClass::new(self.game.BUTTONPLUS, self.ss);
        let mut tsubpart3: SubPartClass = ButtonPartClass::new(self.game.BUTTONPLUS, self.ss);
        self.BAddUnitId = self.AddSubPart(tsubpart3, 320, 60, 32, 16, 1);
        let mut tsubpart4: SubPartClass = TextPartClass::new("Add Action Card", Font::new("Times New Roman", 16, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, &self.ss);
        self.BAddUnitTextId = self.AddSubPart(tsubpart4, 350, 59, 200, 20, 0);
        self.ss = "Click to import ALL actioncards from a different scenario".to_string();
        let mut tsubpart5: SubPartClass = ButtonPartClass::new(self.game.BUTTONBLUE, self.ss);
        self.b2id = self.AddSubPart(tsubpart5, 620, 60, 32, 16, 1);
        let mut tsubpart6: SubPartClass = TextPartClass::new("Import ALL cards", Font::new("Times New Roman", 16, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, &self.ss);
        self.b2bid = self.AddSubPart(tsubpart6, 650, 59, 200, 20, 0);
    }

    pub fn MakeUnitTypeItemGUI(mut self) {
        if self.BNameId > 0 {
            self.RemoveSubPart(self.BNameId);
        }
        if self.BNameTextId > 0 {
            self.RemoveSubPart(self.BNameTextId);
        }
        if self.BRemoveUnitId > 0 {
            self.RemoveSubPart(self.BRemoveUnitId);
        }
        if (self.BRemoveUnitTextId > 0) {
            self.RemoveSubPart(self.BRemoveUnitTextId);
        }
        if (self.cloneid > 0) {
            self.RemoveSubPart(self.cloneid);
        }
        if (self.clonetextid > 0) {
            self.RemoveSubPart(self.clonetextid);
        }
        if (self.x1id > 0) {
            self.RemoveSubPart(self.x1id);
        }
        if (self.x2id > 0) {
            self.RemoveSubPart(self.x2id);
        }
        if (self.x3id > 0) {
            self.RemoveSubPart(self.x3id);
        }
        if (self.b1id > 0) {
            self.RemoveSubPart(self.b1id);
        }
        if (self.b1bid > 0) {
            self.RemoveSubPart(self.b1bid);
        }
        if (self.b3id > 0) {
            self.RemoveSubPart(self.b3id);
        }
        if (self.b3bid > 0) {
            self.RemoveSubPart(self.b3bid);
        }
        if (self.b4id > 0) {
            self.RemoveSubPart(self.b4id);
        }
        if (self.b4bid > 0) {
            self.RemoveSubPart(self.b4bid);
        }
        if (self.a1id > 0) {
            self.RemoveSubPart(self.a1id);
        }
        if (self.a1bid > 0) {
            self.RemoveSubPart(self.a1bid);
        }
        if (self.a2id > 0) {
            self.RemoveSubPart(self.a2id);
        }
        if (self.a2bid > 0) {
            self.RemoveSubPart(self.a2bid);
        }
        if (self.a3id > 0) {
            self.RemoveSubPart(self.a3id);
        }
        if (self.a3bid > 0) {
            self.RemoveSubPart(self.a3bid);
        }
        if (self.a4id > 0) {
            self.RemoveSubPart(self.a4id);
        }
        if (self.a4bid > 0) {
            self.RemoveSubPart(self.a4bid);
        }
        if (self.a5id > 0) {
            self.RemoveSubPart(self.a5id);
        }
        if (self.a5bid > 0) {
            self.RemoveSubPart(self.a5bid);
        }
        if (self.aa4id > 0) {
            self.RemoveSubPart(self.aa4id);
        }
        if (self.aa4bid > 0) {
            self.RemoveSubPart(self.aa4bid);
        }
        if (self.aa5id > 0) {
            self.RemoveSubPart(self.aa5id);
        }
        if (self.aa5bid > 0) {
            self.RemoveSubPart(self.aa5bid);
        }
        if (self.a6id > 0) {
            self.RemoveSubPart(self.a6id);
        }
        if (self.a6bid > 0) {
            self.RemoveSubPart(self.a6bid);
        }
        if (self.a7id > 0) {
            self.RemoveSubPart(self.a7id);
        }
        if (self.a7bid > 0) {
            self.RemoveSubPart(self.a7bid);
        }
        if (self.a8id > 0) {
            self.RemoveSubPart(self.a8id);
        }
        if (self.a8bid > 0) {
            self.RemoveSubPart(self.a8bid);
        }
        if (self.a9id > 0) {
            self.RemoveSubPart(self.a9id);
        }
        if (self.a9bid > 0) {
            self.RemoveSubPart(self.a9bid);
        }
        if (self.a10id > 0) {
            self.RemoveSubPart(self.a10id);
        }
        if (self.a10bid > 0) {
            self.RemoveSubPart(self.a10bid);
        }
        if (self.a11id > 0) {
            self.RemoveSubPart(self.a11id);
        }
        if (self.a11bid > 0) {
            self.RemoveSubPart(self.a11bid);
        }
        if (self.a12id > 0) {
            self.RemoveSubPart(self.a12id);
        }
        if (self.a12bid > 0) {
            self.RemoveSubPart(self.a12bid);
        }
        if (self.a13id > 0) {
            self.RemoveSubPart(self.a13id);
        }
        if (self.a13bid > 0) {
            self.RemoveSubPart(self.a13bid);
        }
        if (self.aa13id > 0) {
            self.RemoveSubPart(self.aa13id);
        }
        if (self.aa13bid > 0) {
            self.RemoveSubPart(self.aa13bid);
        }
        if (self.a14id > 0) {
            self.RemoveSubPart(self.a14id);
        }
        if (self.a14bid > 0) {
            self.RemoveSubPart(self.a14bid);
        }
        if (self.a15id > 0) {
            self.RemoveSubPart(self.a15id);
        }
        if (self.a15bid > 0) {
            self.RemoveSubPart(self.a15bid);
        }
        if (self.a16id > 0) {
            self.RemoveSubPart(self.a16id);
        }
        if (self.a16bid > 0) {
            self.RemoveSubPart(self.a16bid);
        }
        if (self.a17id > 0) {
            self.RemoveSubPart(self.a17id);
        }
        if (self.a17bid > 0) {
            self.RemoveSubPart(self.a17bid);
        }
        if (self.a18id > 0) {
            self.RemoveSubPart(self.a18id);
        }
        if (self.a18bid > 0) {
            self.RemoveSubPart(self.a18bid);
        }
        if (self.a19id > 0) {
            self.RemoveSubPart(self.a19id);
        }
        if (self.a19bid > 0) {
            self.RemoveSubPart(self.a19bid);
        }
        if (self.a20id > 0) {
            self.RemoveSubPart(self.a20id);
        }
        if (self.a20bid > 0) {
            self.RemoveSubPart(self.a20bid);
        }
        if (self.a21id > 0) {
            self.RemoveSubPart(self.a21id);
        }
        if (self.a21bid > 0) {
            self.RemoveSubPart(self.a21bid);
        }
        if (self.a22id > 0) {
            self.RemoveSubPart(self.a22id);
        }
        if (self.a22bid > 0) {
            self.RemoveSubPart(self.a22bid);
        }
        if (self.a23id > 0) {
            self.RemoveSubPart(self.a23id);
        }
        if (self.a23bid > 0) {
            self.RemoveSubPart(self.a23bid);
        }
        if (self.UnitList2Id > 0) {
            self.RemoveSubPart(self.UnitList2Id);
        }
        if (self.UnitNr <= -1) {
            return;
        }
        self.ss = "".to_string();
        let mut tsubpart1: SubPartClass = ButtonPartClass::new(self.game.BUTTONBLOCK, self.ss);
        self.BNameId = self.AddSubPart(tsubpart1, 320, 200, 32, 16, 1);
        let mut tsubpart2: SubPartClass = TextPartClass::new("Title: " + self.game.Data.ActionCardObj[self.UnitNr].Title + " (#" + Strings.Trim(Conversion.Str(self.UnitNr)) + ")", Font::new("Times New Roman", 16, FontStyle.Regular, GraphicsUnit.Pixel), 500, 20, false, &self.ss);
        self.BNameTextId = self.AddSubPart(tsubpart2, 350, 199, 500, 20, 0);
        self.ss = "".to_string();
        tsubpart2 = ButtonPartClass::new(self.game.BUTTONBLOCK, self.ss);
        self.a1id = self.AddSubPart(tsubpart2, 320, 240, 32, 16, 1);
        tsubpart2 = TextPartClass::new("PPCost: " + Conversion.Str(self.game.Data.ActionCardObj[self.UnitNr].PPCost), Font::new("Times New Roman", 16, FontStyle.Regular, GraphicsUnit.Pixel), 500, 20, false, &self.ss);
        self.a1bid = self.AddSubPart(tsubpart2, 350, 239, 500, 20, 0);
        self.ss = "".to_string();
        tsubpart2 = ButtonPartClass::new(self.game.BUTTONBLOCK, self.ss);
        self.a2id = self.AddSubPart(tsubpart2, 320, 220, 32, 16, 1);
        tsubpart2 = TextPartClass::new("Text: " + self.game.Data.ActionCardObj[self.UnitNr].Text, Font::new("Times New Roman", 16, FontStyle.Regular, GraphicsUnit.Pixel), 500, 20, false, &self.ss);
        self.a2bid = self.AddSubPart(tsubpart2, 350, 219, 500, 20, 0);
        self.ss = "".to_string();
        tsubpart2 = ButtonPartClass::new(self.game.BUTTONBLOCK, self.ss);
        self.a3id = self.AddSubPart(tsubpart2, 320, 260, 32, 16, 1);
        tsubpart2 = TextPartClass::new("ExecuteEvent: " + Conversion.Str(self.game.Data.ActionCardObj[self.UnitNr].ExecuteEvent), Font::new("Times New Roman", 16, FontStyle.Regular, GraphicsUnit.Pixel), 500, 20, false, &self.ss);
        self.a3bid = self.AddSubPart(tsubpart2, 350, 259, 500, 20, 0);
        self.ss = "Give event pic # to set the picture used by the actioncards".to_string();
        tsubpart2 = ButtonPartClass::new(self.game.BUTTONBLOCK, self.ss);
        self.a4id = self.AddSubPart(tsubpart2, 320, 280, 32, 16, 1);
        tsubpart2 = TextPartClass::new("EventPicNr: " + Conversion.Str(self.game.Data.ActionCardObj[self.UnitNr].EventPicNr), Font::new("Times New Roman", 16, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, &self.ss);
        self.a4bid = self.AddSubPart(tsubpart2, 350, 279, 200, 20, 0);
        if ((self.game.Data.ActionCardObj[self.UnitNr].EventPicNr > -1) && (self.game.Data.ActionCardObj[self.UnitNr].EventPicNr <= self.game.Data.EventPicCounter)) {
            tsubpart2 = ButtonPartClass::new(self.game.Data.EventPicNr[self.game.Data.ActionCardObj[self.UnitNr].EventPicNr], 150, 150);
            self.aa5id = self.AddSubPart(tsubpart2, 350, 500, 150, 150, 0);
        }
        self.ss = "Regimes with AlternativeActionCardPics=true use this alterantive event pic #".to_string();
        tsubpart2 = ButtonPartClass::new(self.game.BUTTONBLOCK, self.ss);
        self.aa4id = self.AddSubPart(tsubpart2, 620, 280, 32, 16, 1);
        tsubpart2 = TextPartClass::new("Alt. EventPicNr: " + Conversion.Str(self.game.Data.ActionCardObj[self.UnitNr].AlternateEventPicNr), Font::new("Times New Roman", 16, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, &self.ss);
        self.aa4bid = self.AddSubPart(tsubpart2, 650, 279, 200, 20, 0);
        if ((self.game.Data.ActionCardObj[self.UnitNr].EventPicNr > -1) && (self.game.Data.ActionCardObj[self.UnitNr].EventPicNr <= self.game.Data.EventPicCounter) && (self.game.Data.ActionCardObj[self.UnitNr].AlternateEventPicNr > -1)) {
            tsubpart2 = ButtonPartClass::new(self.game.Data.EventPicNr[self.game.Data.ActionCardObj[self.UnitNr].AlternateEventPicNr], 150, 150);
            self.aa5bid = self.AddSubPart(tsubpart2, 650, 500, 150, 150, 0);
        }
        self.ss = "".to_string();
        tsubpart2 = ButtonPartClass::new(self.game.BUTTONBLOCK, self.ss);
        self.a5id = self.AddSubPart(tsubpart2, 320, 300, 32, 16, 1);
        tsubpart2 = TextPartClass::new("ColorScheme: " + Conversion.Str(self.game.Data.ActionCardObj[self.UnitNr].ColorScheme), Font::new("Times New Roman", 16, FontStyle.Regular, GraphicsUnit.Pixel), 500, 20, false, &self.ss);
        self.a5bid = self.AddSubPart(tsubpart2, 350, 299, 500, 20, 0);
        self.ss = "If this card is with a historicalunit officer the AI will play the card randomly if 0. >0 it will play event to determine target unit(areax) or hex(areax,areay)".to_string();
        tsubpart2 = ButtonPartClass::new(self.game.BUTTONBLOCK, self.ss);
        self.a6id = self.AddSubPart(tsubpart2, 320, 320, 32, 16, 1);
        tsubpart2 = TextPartClass::new("AILabel: " + (Conversion.Str(self.game.Data.ActionCardObj[self.UnitNr].AILabel) + ", " + Conversion.Str(self.game.Data.ActionCardObj[self.UnitNr].AILabel2) + ", " + Conversion.Str(self.game.Data.ActionCardObj[self.UnitNr].aILabel3)), Font::new("Times New Roman", 16, FontStyle.Regular, GraphicsUnit.Pixel), 500, 20, false, &self.ss);
        self.a6bid = self.AddSubPart(tsubpart2, 350, 319, 500, 20, 0);
        self.ss = "tempvar0,tempvar1 will be passed to event.".to_string();
        tsubpart2 = ButtonPartClass::new(self.game.BUTTONBLOCK, self.ss);
        self.a12id = self.AddSubPart(tsubpart2, 320, 340, 32, 16, 1);
        tsubpart2 = TextPartClass::new("TempVar0: " + Conversion.Str(self.game.Data.ActionCardObj[self.UnitNr].TempVar0), Font::new("Times New Roman", 16, FontStyle.Regular, GraphicsUnit.Pixel), 100, 20, false, &self.ss);
        self.a12bid = self.AddSubPart(tsubpart2, 350, 339, 100, 20, 0);
        self.ss = "tempvar0,tempvar1 will be passed to event.".to_string();
        tsubpart2 = ButtonPartClass::new(self.game.BUTTONBLOCK, self.ss);
        self.a13id = self.AddSubPart(tsubpart2, 480, 340, 32, 16, 1);
        tsubpart2 = TextPartClass::new("TempVar1: " + Conversion.Str(self.game.Data.ActionCardObj[self.UnitNr].TempVar1), Font::new("Times New Roman", 16, FontStyle.Regular, GraphicsUnit.Pixel), 100, 20, false, &self.ss);
        self.a13bid = self.AddSubPart(tsubpart2, 510, 339, 100, 20, 0);
        self.ss = "do nothing special=0, 1=only show card with corps, 2=only show card with army or higher".to_string();
        tsubpart2 = ButtonPartClass::new(self.game.BUTTONBLOCK, self.ss);
        self.aa13id = self.AddSubPart(tsubpart2, 320, 360, 32, 16, 1);
        tsubpart2 = TextPartClass::new("LimitedOfficerShow: " + Conversion.Str(self.game.Data.ActionCardObj[self.UnitNr].LimitedShow), Font::new("Times New Roman", 16, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, &self.ss);
        self.aa13bid = self.AddSubPart(tsubpart2, 350, 359, 300, 20, 0);
        self.ss = "Specify -1 if card has no hisvar cost of any kind".to_string();
        tsubpart2 = ButtonPartClass::new(self.game.BUTTONBLOCK, self.ss);
        self.b3id = self.AddSubPart(tsubpart2, 620, 340, 32, 16, 1);
        tsubpart2 = TextPartClass::new("HisVarType: " + Conversion.Str(self.game.Data.ActionCardObj[self.UnitNr].HisVarCostType), Font::new("Times New Roman", 16, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, &self.ss);
        self.b3bid = self.AddSubPart(tsubpart2, 650, 339, 200, 20, 0);
        self.ss = "Specify the cost in points for the HisVarType specified. ".to_string();
        tsubpart2 = ButtonPartClass::new(self.game.BUTTONBLOCK, self.ss);
        self.b4id = self.AddSubPart(tsubpart2, 620, 360, 32, 16, 1);
        tsubpart2 = TextPartClass::new("HisVarCost: " + Conversion.Str(self.game.Data.ActionCardObj[self.UnitNr].HisVarCostQty), Font::new("Times New Roman", 16, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, &self.ss);
        self.b4bid = self.AddSubPart(tsubpart2, 650, 359, 200, 20, 0);
        self.ss = "Defining the highlighted hexes for user pic. User selection will be passed as tempvar2,tempvar3 to event.".to_string();
        tsubpart2 = ButtonPartClass::new(self.game.BUTTONBLOCK, self.ss);
        self.a14id = self.AddSubPart(tsubpart2, 320, 380, 32, 16, 1);
        tsubpart2 = TextPartClass::new("AreaSlot: " + Conversion.Str(self.game.Data.ActionCardObj[self.UnitNr].AreaSlot), Font::new("Times New Roman", 16, FontStyle.Regular, GraphicsUnit.Pixel), 500, 20, false, &self.ss);
        self.a14bid = self.AddSubPart(tsubpart2, 350, 379, 500, 20, 0);
        self.ss = "Defining the highlighted hexes for user pic. User selection will be passed as tempvar2,tempvar3 to event.".to_string();
        tsubpart2 = ButtonPartClass::new(self.game.BUTTONBLOCK, self.ss);
        self.a15id = self.AddSubPart(tsubpart2, 320, 400, 32, 16, 1);
        tsubpart2 = TextPartClass::new("AreaValue: " + Conversion.Str(self.game.Data.ActionCardObj[self.UnitNr].AreaValue), Font::new("Times New Roman", 16, FontStyle.Regular, GraphicsUnit.Pixel), 500, 20, false, &self.ss);
        self.a15bid = self.AddSubPart(tsubpart2, 350, 399, 500, 20, 0);
        self.ss = "-1=for none. Pre Execute event is executed the moment before you select something on the popup. Use to define selectable zones. Or set Selectable Units".to_string();
        tsubpart2 = ButtonPartClass::new(self.game.BUTTONBLOCK, self.ss);
        self.a16id = self.AddSubPart(tsubpart2, 320, 420, 32, 16, 1);
        tsubpart2 = TextPartClass::new("PreExecuteEvent: " + Conversion.Str(self.game.Data.ActionCardObj[self.UnitNr].PreExecuteEvent), Font::new("Times New Roman", 16, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, &self.ss);
        self.a16bid = self.AddSubPart(tsubpart2, 350, 419, 200, 20, 0);
        self.ss = "Determines if this event will give you a popup to select a unit.".to_string();
        tsubpart2 = ButtonPartClass::new(self.game.BUTTONBLOCK, self.ss);
        self.a17id = self.AddSubPart(tsubpart2, 320, 440, 32, 16, 1);
        tsubpart2 = TextPartClass::new("SelectUnit: " + Conversion.Str(self.game.Data.ActionCardObj[self.UnitNr].UnitSelect), Font::new("Times New Roman", 16, FontStyle.Regular, GraphicsUnit.Pixel), 500, 20, false, &self.ss);
        self.a17bid = self.AddSubPart(tsubpart2, 350, 439, 500, 20, 0);
        self.ss = "The minisprite is used to draw the small officer cards. but you dont have to specify. -1=dont use.".to_string();
        tsubpart2 = ButtonPartClass::new(self.game.BUTTONBLOCK, self.ss);
        self.a18id = self.AddSubPart(tsubpart2, 320, 460, 32, 16, 1);
        tsubpart2 = TextPartClass::new("Nato Mini Sprite: " + Conversion.Str(self.game.Data.ActionCardObj[self.UnitNr].Nato), Font::new("Times New Roman", 16, FontStyle.Regular, GraphicsUnit.Pixel), 500, 20, false, &self.ss);
        self.a18bid = self.AddSubPart(tsubpart2, 350, 459, 500, 20, 0);
        self.ss = "The mouse over text will appear if new GUI is used and card is right clicked.".to_string();
        tsubpart2 = ButtonPartClass::new(self.game.BUTTONBLOCK, self.ss);
        self.a19id = self.AddSubPart(tsubpart2, 320, 480, 32, 16, 1);
        tsubpart2 = TextPartClass::new("Mouse Over " + self.game.Data.ActionCardObj[self.UnitNr].MouseOver, Font::new("Times New Roman", 16, FontStyle.Regular, GraphicsUnit.Pixel), 250, 20, false, &self.ss);
        self.a19bid = self.AddSubPart(tsubpart2, 350, 479, 250, 20, 0);
        self.ss = "Card Category. Also see rulevar 905 to activate this in GUI. ".to_string();
        tsubpart2 = ButtonPartClass::new(self.game.BUTTONBLOCK, self.ss);
        self.a20id = self.AddSubPart(tsubpart2, 620, 420, 32, 16, 1);
        tsubpart2 = TextPartClass::new("Category =" + self.game.Data.ActionCardObj[self.UnitNr].Category.ToString(), Font::new("Times New Roman", 16, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, &self.ss);
        self.a20bid = self.AddSubPart(tsubpart2, 650, 419, 200, 20, 0);
        self.ss = "Change library".to_string();
        tsubpart2 = ButtonPartClass::new(self.game.BUTTONBLOCK, self.ss);
        self.a21id = self.AddSubPart(tsubpart2, 620, 440, 32, 16, 1);
        tsubpart2 = TextPartClass::new("Set Library" + self.game.Data.ActionCardObj[self.UnitNr].LibId.libSlot.ToString(), Font::new("Times New Roman", 16, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, &self.ss);
        self.a21bid = self.AddSubPart(tsubpart2, 650, 439, 200, 20, 0);
        self.ss = "Set Small Gfx.  ".to_string();
        tsubpart2 = ButtonPartClass::new(self.game.BUTTONBLOCK, self.ss);
        self.a22id = self.AddSubPart(tsubpart2, 620, 460, 32, 16, 1);
        tsubpart2 = TextPartClass::new("Small Gfx =" + self.game.Data.ActionCardObj[self.UnitNr].SmallGfx.ToString(), Font::new("Times New Roman", 16, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, &self.ss);
        self.a22bid = self.AddSubPart(tsubpart2, 650, 459, 200, 20, 0);
        self.ss = "Ignore Popup if no selection possible of either units or hexes. The actual event can read if its skipped by calling CheckPopupSkipped().  ".to_string();
        tsubpart2 = ButtonPartClass::new(self.game.BUTTONBLOCK, self.ss);
        self.a23id = self.AddSubPart(tsubpart2, 620, 480, 32, 16, 1);
        tsubpart2 = TextPartClass::new("IgnorePopupIfNoSel =" + self.game.Data.ActionCardObj[self.UnitNr].IgnorePopupIfNoSelect.ToString(), Font::new("Times New Roman", 16, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, &self.ss);
        self.a23bid = self.AddSubPart(tsubpart2, 650, 479, 200, 20, 0);
        self.ss = "Click to remove this action card.".to_string();
        tsubpart2 = ButtonPartClass::new(self.game.BUTTONKILL, self.ss);
        self.BRemoveUnitId = self.AddSubPart(tsubpart2, 320, 80, 32, 16, 1);
        tsubpart2 = TextPartClass::new("Remove this Action Card", Font::new("Times New Roman", 16, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, &self.ss);
        self.BRemoveUnitTextId = self.AddSubPart(tsubpart2, 350, 79, 200, 20, 0);
        tsubpart2 = ButtonPartClass::new(self.game.CustomBitmapObj.DrawActionCardMarc2(0, self.UnitNr));
        self.x1id = self.AddSubPart(tsubpart2, 830, 100, 190, 266, 0);
        tsubpart2 = ButtonPartClass::new(self.game.CustomBitmapObj.DrawActionCardMarc2(0, self.UnitNr, 2));
        self.x2id = self.AddSubPart(tsubpart2, 830, 400, 105, 147, 0);
        tsubpart2 = ButtonPartClass::new(self.game.CustomBitmapObj.DrawActionCardMarc2(0, self.UnitNr, 3));
        self.x3id = self.AddSubPart(tsubpart2, 830, 600, 33, 46, 0);
    }

    pub fn HandleMouseClick(mut self, x: i32, y: i32, b: i32) -> WindowReturnClass {
        let mut windowReturnClass: WindowReturnClass = WindowReturnClass::new();
        if self.SubPartCounter > -1 {
            let mut subPartCounter: i32 = self.SubPartCounter;
            // for (let mut index1: i32 = 0; index1 <= subPartCounter; index1 += 1)
            let mut index1: i32 = 0;
            while (index1 <= subPartCounter) {
                if ((x > self.SubPartX[index1]) && (x < self.SubPartX[index1] + self.SubPartW[index1]) && (y > self.SubPartY[index1]) && (y < self.SubPartY[index1] + self.SubPartH[index1])) {
                    let mut num1: i32 = self.SubPartID[index1];
                    if num1 == self.libListId {
                        let mut num2: i32 = self.SubPartList[index1].Click(x - self.SubPartX[index1], y - self.SubPartY[index1]);
                        self.SubPartFlag[index1] = true;
                        if num2 > -1 {
                            self.libnr = num2;
                            self.UnitNr = -1;
                            self.MakeUnitListGUI();
                        } else if num2 == -2 {
                            self.libnr = -1;
                            self.UnitNr = -1;
                            self.MakeUnitListGUI();
                        }
                        windowReturnClass.SetFlag(true);
                        return windowReturnClass;
                    }
                    if (num1 == self.UnitListId) {
                        let mut num3: i32 = self.SubPartList[index1].Click(x - self.SubPartX[index1], y - self.SubPartY[index1]);
                        self.SubPartFlag[index1] = true;
                        if (num3 > -1) {
                            self.UnitNr = num3;
                            self.MakeUnitTypeItemGUI();
                        }
                        windowReturnClass.SetFlag(true);
                        return windowReturnClass;
                    }
                    if (num1 == self.BAddUnitId) {
                        self.game.Data.AddActionCard();
                        self.UnitNr = self.game.Data.ActionCardCounter;
                        self.MakeUnitListGUI();
                        windowReturnClass.SetFlag(true);
                        return windowReturnClass;
                    }
                    if num1 == self.a1id {
                        let mut num4: i32 = Math.Round(Conversion.Val(Interaction.InputBox("Set PPCost", "Shadow Empire : Planetary Conquest")));
                        if (num4 > -9999) && (num4 < 9999) {
                            self.game.Data.ActionCardObj[self.UnitNr].PPCost = num4;
                        }
                        self.MakeUnitListGUI();
                        windowReturnClass.SetFlag(true);
                        return windowReturnClass;
                    }
                    if num1 == self.a12id {
                        let mut num5: i32 = Math.Round(Conversion.Val(Interaction.InputBox("Set TempVar0 (-1=dont set)", "Shadow Empire : Planetary Conquest")));
                        if (num5 > -99999) && (num5 < 99999) {
                            self.game.Data.ActionCardObj[self.UnitNr].TempVar0 = num5;
                        }
                        self.MakeUnitListGUI();
                        windowReturnClass.SetFlag(true);
                        return windowReturnClass;
                    }
                    if (num1 == self.b2id) {
                        let str1: String = self.game.HandyFunctionsObj.LoadSomething("SE1 Scenario file (*.se1)|*.se1", "Pick a scenario to load all stringlists from...", self.game.AppPath + "scenarios\\", false);
                        let path = Path::new(&str1);
                        if path.exists() {
                            self.game.HandyFunctionsObj.Unzip(&str1);
                            let mut dataClass: DataClass = DataClass.deserialize(&str1);
                            self.game.HandyFunctionsObj.ZipFile(&str1);
                            let mut actionCardCounter: i32 = dataClass.ActionCardCounter;
                            // for (let mut index2: i32 = 0; index2 <= actionCardCounter; index2 += 1)
                            let mut index2: i32 = 0;
                            while index2 <= actionCardCounter {
                                self.game.Data.AddActionCard();
                                self.game.Data.ActionCardObj[self.game.Data.ActionCardCounter] = dataClass.ActionCardObj[index2].Clone();
                                index2 += 1;
                            }
                            self.UnitNr = self.game.Data.ActionCardCounter;
                            self.MakeUnitListGUI();
                        }
                        windowReturnClass.SetFlag(true);
                        return windowReturnClass;
                    }
                    if num1 == self.a13id {
                        let mut num6: i32 = Math.Round(Conversion.Val(Interaction.InputBox("Set TempVar1 (-1=dont set)", "Shadow Empire : Planetary Conquest")));
                        if (num6 > -99999) && (num6 < 99999) {
                            self.game.Data.ActionCardObj[self.UnitNr].TempVar1 = num6;
                        }
                        self.MakeUnitListGUI();
                        windowReturnClass.SetFlag(true);
                        return windowReturnClass;
                    }
                    if num1 == self.a14id {
                        let mut num7: i32 = Math.Round(Conversion.Val(Interaction.InputBox("Set Areaslot(-1=dont set)", "Shadow Empire : Planetary Conquest")));
                        if (num7 > -99999) && (num7 < 99999) {
                            self.game.Data.ActionCardObj[self.UnitNr].AreaSlot = num7;
                        }
                        self.MakeUnitListGUI();
                        windowReturnClass.SetFlag(true);
                        return windowReturnClass;
                    }
                    if num1 == self.aa13id {
                        let mut num8: i32 = Math.Round(Conversion.Val(Interaction.InputBox("0,1 or 2", "Shadow Empire : Planetary Conquest")));
                        if (num8 >= 0) && (num8 <= 2) {
                            self.game.Data.ActionCardObj[self.UnitNr].LimitedShow = num8;
                        }
                        self.MakeUnitListGUI();
                        windowReturnClass.SetFlag(true);
                        return windowReturnClass;
                    }
                    if num1 == self.a23id {
                        self.game.Data.ActionCardObj[self.UnitNr].IgnorePopupIfNoSelect = !self.game.Data.ActionCardObj[self.UnitNr].IgnorePopupIfNoSelect;
                        self.MakeUnitListGUI();
                        windowReturnClass.SetFlag(true);
                        return windowReturnClass;
                    }
                    if num1 == self.a15id {
                        let mut num9: i32 = Math.Round(Conversion.Val(Interaction.InputBox("Set Areavalue(-1=dont set)", "Shadow Empire : Planetary Conquest")));
                        if (num9 > -99999) & (num9 < 99999) {
                            self.game.Data.ActionCardObj[self.UnitNr].AreaValue = num9;
                        }
                        self.MakeUnitListGUI();
                        windowReturnClass.SetFlag(true);
                        return windowReturnClass;
                    }
                    if num1 == self.a18id {
                        let mut num10: i32 = Math.Round(Conversion.Val(Interaction.InputBox("NATO Sprite? (-1=dont set)", "Shadow Empire : Planetary Conquest")));
                        if num10 >= -1 && num10 <= self.game.NATO.GetUpperBound(0) {
                            self.game.Data.ActionCardObj[self.UnitNr].Nato = num10;
                        }
                        self.MakeUnitListGUI();
                        windowReturnClass.SetFlag(true);
                        return windowReturnClass;
                    }
                    if (num1 == self.a22id) {
                        let mut num11: i32 = Math.Round(Conversion.Val(Interaction.InputBox("Small Gfx nr? (-1=dont set)", "Shadow Empire : Planetary Conquest")));
                        if num11 >= -1 && num11 <= self.game.Data.SmallPicCounter {
                            self.game.Data.ActionCardObj[self.UnitNr].SmallGfx = num11;
                        }
                        self.MakeUnitListGUI();
                        windowReturnClass.SetFlag(true);
                        return windowReturnClass;
                    }
                    if (num1 == self.b3id) {
                        let mut num12: i32 = Math.Round(Conversion.Val(Interaction.InputBox("HisVar Type (-1=none)", "Shadow Empire : Planetary Conquest")));
                        if num12 >= -1 && num12 <= 99 {
                            self.game.Data.ActionCardObj[self.UnitNr].HisVarCostType = num12;
                        }
                        self.MakeUnitListGUI();
                        windowReturnClass.SetFlag(true);
                        return windowReturnClass;
                    }
                    if (num1 == self.b4id) {
                        let mut num13: i32 = Math.Round(Conversion.Val(Interaction.InputBox("HisVar Qty", "Shadow Empire : Planetary Conquest")));
                        if num13 >= -1 && num13 <= 99 {
                            self.game.Data.ActionCardObj[self.UnitNr].HisVarCostQty = num13;
                        }
                        self.MakeUnitListGUI();
                        windowReturnClass.SetFlag(true);
                        return windowReturnClass;
                    }
                    if num1 == self.a17id {
                        self.game.Data.ActionCardObj[self.UnitNr].UnitSelect = !self.game.Data.ActionCardObj[self.UnitNr].UnitSelect;
                        self.MakeUnitListGUI();
                        windowReturnClass.SetFlag(true);
                        return windowReturnClass;
                    }
                    if (num1 == self.a2id) {
                        Form2::new(self.form).Initialize(self.game.Data, 4, self.UnitNr);
                        windowReturnClass.SetFlag(true);
                        return windowReturnClass;
                    }
                    if (num1 == self.a19id) {
                        Form2::new(self.form).Initialize(self.game.Data, 9, self.UnitNr);
                        windowReturnClass.SetFlag(true);
                        return windowReturnClass;
                    }
                    if (num1 == self.a3id) {
                        Form3::new(self.form).Initialize(self.game.Data, 31, self.UnitNr);
                        windowReturnClass.SetFlag(true);
                        return windowReturnClass;
                    }
                    if (num1 == self.a21id) {
                        Form3::new(self.form).Initialize(self.game.Data, 109, self.UnitNr);
                        windowReturnClass.SetFlag(true);
                        return windowReturnClass;
                    }
                    if (num1 == self.a16id) {
                        Form3::new(self.form).Initialize(self.game.Data, 35, self.UnitNr);
                        windowReturnClass.SetFlag(true);
                        return windowReturnClass;
                    }
                    if (num1 == self.a4id) {
                        let mut num14: i32 = Math.Round(Conversion.Val(Interaction.InputBox("set eventPICTURE nr (-1=none)", "Shadow Empire : Planetary Conquest")));
                        if (num14 > -2 && num14 <= self.game.Data.EventPicCounter) {
                            self.game.Data.ActionCardObj[self.UnitNr].EventPicNr = num14;
                        }
                        self.MakeUnitListGUI();
                        windowReturnClass.SetFlag(true);
                        return windowReturnClass;
                    }
                    if (num1 == self.aa4id) {
                        let mut num15: i32 = Math.Round(Conversion.Val(Interaction.InputBox("set alt. eventPICTURE nr (-1=none)", "Shadow Empire : Planetary Conquest")));
                        if (num15 > -2 && num15 <= self.game.Data.EventPicCounter) {
                            self.game.Data.ActionCardObj[self.UnitNr].AlternateEventPicNr = num15;
                        }
                        self.MakeUnitListGUI();
                        windowReturnClass.SetFlag(true);
                        return windowReturnClass;
                    }
                    if (num1 == self.a20id) {
                        let mut num16: i32 = Math.Round(Conversion.Val(Interaction.InputBox("Give category # (-1=none, 1-5=category#)", "Shadow Empire : Planetary Conquest")));
                        if (num16 > -2 && num16 <= 5) {
                            self.game.Data.ActionCardObj[self.UnitNr].Category = num16;
                        }
                        self.MakeUnitListGUI();
                        windowReturnClass.SetFlag(true);
                        return windowReturnClass;
                    }
                    if (num1 == self.a5id) {
                        let mut num17: i32 = Math.Round(Conversion.Val(Interaction.InputBox("set colorscheme nr (0,1,2,3,4)", "Shadow Empire : Planetary Conquest")));
                        if (num17 > -1 && num17 <= 999) {
                            self.game.Data.ActionCardObj[self.UnitNr].ColorScheme = num17;
                        }
                        self.MakeUnitListGUI();
                        windowReturnClass.SetFlag(true);
                        return windowReturnClass;
                    }
                    if (num1 == self.a6id) {
                        let mut num18: i32 = Math.Round(Conversion.Val(Interaction.InputBox("set hardcoded ai label (0-99999)", "Shadow Empire : Planetary Conquest")));
                        if (num18 > -1 && num18 <= 99999) {
                            self.game.Data.ActionCardObj[self.UnitNr].AILabel = num18;
                            self.game.Data.ActionCardObj[self.UnitNr].AILabel2 = 0;
                            self.game.Data.ActionCardObj[self.UnitNr].aILabel3 = 0;
                            let mut num19: i32 = Math.Round(Conversion.Val(Interaction.InputBox("set hardcoded ai label 2 nr (0-99999)", "Shadow Empire : Planetary Conquest")));
                            if (num19 > -1 && num19 <= 99999) {
                                self.game.Data.ActionCardObj[self.UnitNr].AILabel2 = num19;
                                let mut num20: i32 = Math.Round(Conversion.Val(Interaction.InputBox("set hardcoded ai label 3 nr (0-99999)", "Shadow Empire : Planetary Conquest")));
                                if (num20 > -1 && num20 <= 99999) {
                                    self.game.Data.ActionCardObj[self.UnitNr].aILabel3 = num20;
                                }
                            }
                        }
                        self.MakeUnitListGUI();
                        windowReturnClass.SetFlag(true);
                        return windowReturnClass;
                    }
                    if (num1 == self.BNameId) {
                        self.game.Data.ActionCardObj[self.UnitNr].Title = Interaction.InputBox("Give new name, please.", "Give Name", self.game.Data.ActionCardObj[self.UnitNr].Title);
                        self.MakeUnitListGUI();
                        windowReturnClass.SetFlag(true);
                        return windowReturnClass;
                    }
                    if (num1 == self.BRemoveUnitId && self.UnitNr > -1) {
                        self.game.Data.RemoveActionCard(self.UnitNr);
                        --self.UnitNr;
                        self.MakeUnitListGUI();
                        windowReturnClass.SetFlag(true);
                        return windowReturnClass;
                    }
                }
                index += 1;
            }
            windowReturnClass.SetFlag(false);
            return windowReturnClass;
        }
        windowReturnClass.SetFlag(false);
        return windowReturnClass;
    }
}
// }
