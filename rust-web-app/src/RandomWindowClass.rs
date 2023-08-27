// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.RandomWindowClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;
// usingSystem.Drawing.Drawing2D;
// usingSystem.Drawing.Text;
// usingSystem.IO;
// usingSystem.Runtime.CompilerServices;
// usingSystem.Windows.Forms;

namespace WindowsApplication1
{
  pub class RandomWindowClass : WindowClass
  {
     BStartGameID: i32;
     BLoadGameID: i32;
     BLadderID: i32;
     BSaveGameID: i32;
     BEditorID: i32;
     TempText: i32;
     TempText2: i32;
     txt1: i32;
     txt2: i32;
     tab1: i32;
     tab2: i32;
     tabmode: i32;
     txt3: i32;
     shrowd: i32;
     doshrowd: i32;
     maploop: i32;
     singlestart: i32;
     dosinglestart: i32;
     dostats: i32;
     optimize: i32;
     dooptimize: i32;
     optimizetext: i32;
     oldkingdomtext: i32;
     domaploop: i32;
     dooldkingdom: i32;
     oldkingdom: i32;
     maplooptext: i32;
     doallied: i32;
     allied: i32;
     alliedtext: i32;
     shrowdtext: i32;
     mirror: i32;
     domirror: i32;
     mirrortext: i32;
     blockcenter: i32;
     dofirsttech: i32;
     firsttech: i32;
     firsttechtext: i32;
     doblockcenter: i32;
     blockcentertext: i32;
     opt1: i32;
     opt2: i32;
     opt3: i32;
     opt4: i32;
     opt5: i32;
     opt6: i32;
     opt7: i32;
     opt8: i32;
     opt9: i32;
     opt10: i32;
     opt11: i32;
     opt12: i32;
     cancelID: i32;
     o1: i32;
     o2: i32;
     o3: i32;
     o16: i32;
     o17: i32;
     o4: i32;
     o5: i32;
     o18: i32;
     o6: i32;
     o7: i32;
     o8: i32;
     o9: i32;
     o10: i32;
     o11: i32;
     o12: i32;
     o13: i32;
     o14: i32;
     o15: i32;
     o19: i32;
     w1: i32;
     w2: i32;
     w3: i32;
     w4: i32;
     w5: i32;
     w6: i32;
     w7: i32;
     w8: i32;
     w9: i32;
     w10: i32;
     w11: i32;
     w12: i32;
     w13: i32;
     w14: i32;
     w15: i32;
     w16: i32;
     w17: i32;
     w21: i32;
     r1: i32;
     r2: i32;
     r3: i32;
     r4: i32;
     r5: i32;
     r6: i32;
     r7: i32;
     r8: i32;
     r9: i32;
     r10: i32;
     r11: i32;
     r12: i32;
     r13: i32;
     r14: i32;
     r15: i32;
     r16: i32;
     r17: i32;
     r21: i32;
     r26: i32;
     r27: i32;
     r28: i32;
     h1: i32;
     h2: i32;
     h3: i32;
     h4: i32;
     h5: i32;
     h6: i32;
     h7: i32;
     h8: i32;
     h9: i32;
     h10: i32;
     h11: i32;
     h12: i32;
     optr1: i32;
     optr2: i32;
     optr3: i32;
     optr4: i32;
     optr5: i32;
     optr6: i32;
     optr7: i32;
     optr8: i32;
     z1: i32;
     z2: i32;
     Srawuse: i32;
     ListClass RegimeListObj;
     RegimeListId: i32;
     float tempBlink;
     detailnr: i32;
     totvp: i32;
     opt1v: i32;
     opt2v: i32;
     opt3v: i32;
     opt4v: i32;
     opt5v: i32;
     opt6v: i32;
     opt7v: i32;
     opt8v: i32;
     opt9v: i32;
     opt10v: i32;
     opt11v: i32;
     opt12v: i32;
     WATER: i32;
     int[] TownSize;
     bool[] TownCapitol;
     GRASS: i32;
     HIGHMOUNTAIN: i32;
     LOWMOUNTAIN: i32;
     LIGHTFOREST: i32;
     HEAVYFOREST: i32;
     SMALLRIVER: i32;
     URBAN: i32;
     LIGHTURBAN: i32;
     FARMLAND: i32;
     BIGRIVER: i32;
     SWAMP: i32;
     landcur: i32;
     mountaincur: i32;
     forestcur: i32;
     rivercur: i32;
     int[] tempx;
     int[] tempy;
     tempcount: i32;
     namelist: Vec<String>;
     namecount: i32;
     domasterfile: String;
     masterfile: i32;
     masterfiletext: i32;
     Flag1: String;
     Flag1b: String;
     int[,,] curriv;
     int[,,] rivstep;
     Coordinate[,,] nextrivstep;
     RESOURCESLOT: i32;
     Sworldsize: i32;
     Splayer: i32;
     Swater: i32;
     Sclimate: i32;
     Scrate: i32;
     Spop: i32;
     Sraw: i32;
     int[] RegFavClimate;
     int[] Regid;
     object[,] town;
     object[,] town2;

    pub RandomWindowClass( tGame: GameClass, bool Marc)
      : base( tGame, 1024, 768, BackSprite: tGame.BACKGROUND2MARC)
    {
      self.TownSize = new int[10000];
      self.TownCapitol = new bool[10000];
      self.tempx = new int[1001];
      self.tempy = new int[1001];
      self.namelist = new string[1001];
      self.curriv = new int[2, 2, 2];
      self.rivstep = new int[2, 2, 2];
      self.nextrivstep = new Coordinate[2, 2, 2];
      self.RegFavClimate = new int[100];
      self.Regid = new int[100];
      self.town = new object[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      self.town2 = new object[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      self.RandomSetup();
      self.DoStuff();
    }

    pub RandomWindowClass( tGame: GameClass)
      : base( tGame, 1024, 768, BackSprite: tGame.BACKGROUND2MARC)
    {
      self.TownSize = new int[10000];
      self.TownCapitol = new bool[10000];
      self.tempx = new int[1001];
      self.tempy = new int[1001];
      self.namelist = new string[1001];
      self.curriv = new int[2, 2, 2];
      self.rivstep = new int[2, 2, 2];
      self.nextrivstep = new Coordinate[2, 2, 2];
      self.RegFavClimate = new int[100];
      self.Regid = new int[100];
      self.town = new object[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      self.town2 = new object[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      self.RandomSetup();
      self.DoStuff();
    }

    pub fn RandomSetup()
    {
      self.tempBlink = 0.0f;
      self.detailnr = -1;
      self.opt1v = 20;
      self.opt2v = 20;
      self.opt3v = 2;
      self.opt4v = 100;
      self.opt5v = 0;
      self.opt6v = 30;
      self.opt7v = 25;
      self.opt8v = 5;
      self.opt9v = 2;
      self.opt10v = 2;
      self.opt11v = 100;
      self.doshrowd = 0;
      self.domirror = 0;
      self.dofirsttech = 1;
      self.doallied = 0;
      self.domaploop = 0;
      self.doblockcenter = 0;
      self.domasterfile = self.game.EditObj.RandomUseMaster;
      self.dooldkingdom = 0;
      self.dooptimize = 0;
      self.singlestart = 0;
      self.dosinglestart = 0;
      self.dostats = 1;
      self.Scrate = 1;
      self.Spop = 0;
      self.Sraw = 0;
      self.Srawuse = 1;
      if (self.game.EditObj.ranmem == 1)
      {
        self.optr1 = self.game.EditObj.ranr1;
        self.optr2 = self.game.EditObj.ranr2;
        self.optr3 = self.game.EditObj.ranr3;
        self.optr4 = self.game.EditObj.ranr4;
        self.optr5 = self.game.EditObj.ranr5;
        self.optr6 = self.game.EditObj.ranr6;
        self.optr7 = self.game.EditObj.ranr7;
        self.optr8 = self.game.EditObj.ranr8;
        self.opt1v = self.game.EditObj.ran1;
        self.opt2v = self.game.EditObj.ran2;
        self.opt3v = self.game.EditObj.ran3;
        self.opt4v = self.game.EditObj.ran4;
        self.opt5v = self.game.EditObj.ran5;
        self.opt6v = self.game.EditObj.ran6;
        self.opt7v = self.game.EditObj.ran7;
        self.opt8v = self.game.EditObj.ran8;
        self.opt9v = self.game.EditObj.ran9;
        self.opt10v = self.game.EditObj.ran10;
        self.opt11v = self.game.EditObj.ran11;
        self.opt12v = self.game.EditObj.ran12;
        self.doshrowd = self.game.EditObj.randoshrowd;
        self.domirror = self.game.EditObj.randomirror;
        self.doblockcenter = self.game.EditObj.randoblockcenter;
        self.dofirsttech = self.game.EditObj.randofirsttech;
        self.doallied = self.game.EditObj.randoallied;
        self.domaploop = self.game.EditObj.randomaploop;
        self.dooldkingdom = self.game.EditObj.ranoldkingdom;
        self.dooptimize = self.game.EditObj.ranoptimize;
        self.dosinglestart = self.game.EditObj.ransinglestart;
        self.Sworldsize = self.game.EditObj.ransworldsize;
        self.Splayer = self.game.EditObj.ransplayer;
        self.Swater = self.game.EditObj.ranswater;
        self.Sclimate = self.game.EditObj.ransclimate;
        self.Scrate = self.game.EditObj.ranscrate;
        self.Spop = self.game.EditObj.ranpop;
        self.Sraw = self.game.EditObj.ranraw;
        self.dostats = self.game.EditObj.ranstats;
        self.Srawuse = self.game.EditObj.ranrawuse;
      }
      self.game.EditObj.ranr1 = self.optr1;
      self.game.EditObj.ranr2 = self.optr2;
      self.game.EditObj.ranr3 = self.optr3;
      self.game.EditObj.ranr4 = self.optr4;
      self.game.EditObj.ranr5 = self.optr5;
      self.game.EditObj.ranr6 = self.optr6;
      self.game.EditObj.ranr7 = self.optr7;
      self.game.EditObj.ranr8 = self.optr8;
      self.game.EditObj.ran1 = self.opt1v;
      self.game.EditObj.ran2 = self.opt2v;
      self.game.EditObj.ran3 = self.opt3v;
      self.game.EditObj.ran4 = self.opt4v;
      self.game.EditObj.ran5 = self.opt5v;
      self.game.EditObj.ran6 = self.opt6v;
      self.game.EditObj.ran7 = self.opt7v;
      self.game.EditObj.ran8 = self.opt8v;
      self.game.EditObj.ran9 = self.opt9v;
      self.game.EditObj.ran10 = self.opt10v;
      self.game.EditObj.ran11 = self.opt11v;
      self.game.EditObj.ran12 = self.opt12v;
      self.game.EditObj.randomaploop = self.domaploop;
      self.game.EditObj.randoshrowd = self.doshrowd;
      self.game.EditObj.randomirror = self.domirror;
      self.game.EditObj.randoblockcenter = self.doblockcenter;
      self.game.EditObj.randofirsttech = self.dofirsttech;
      self.game.EditObj.ranmasterfile = self.domasterfile;
      self.game.EditObj.randoallied = self.doallied;
      self.game.EditObj.ranoldkingdom = self.dooldkingdom;
      self.game.EditObj.ranoptimize = self.dooptimize;
      self.game.EditObj.ransinglestart = self.dosinglestart;
      self.game.EditObj.ransworldsize = self.Sworldsize;
      self.game.EditObj.ransplayer = self.Splayer;
      self.game.EditObj.ranswater = self.Swater;
      self.game.EditObj.ransclimate = self.Sclimate;
      self.game.EditObj.ranscrate = self.Scrate;
      self.game.EditObj.ranstats = self.dostats;
      self.game.EditObj.ranraw = self.Sraw;
      self.game.EditObj.ranpop = self.Spop;
      self.game.EditObj.ranrawuse = self.Srawuse;
    }

    pub fn DoStuffShort()
    {
      if (self.o1 > 0)
        self.RemoveSubPart(self.o1);
      if (self.o2 > 0)
        self.RemoveSubPart(self.o2);
      if (self.o3 > 0)
        self.RemoveSubPart(self.o3);
      if (self.o4 > 0)
        self.RemoveSubPart(self.o4);
      if (self.o5 > 0)
        self.RemoveSubPart(self.o5);
      if (self.o6 > 0)
        self.RemoveSubPart(self.o6);
      if (self.o7 > 0)
        self.RemoveSubPart(self.o7);
      if (self.o8 > 0)
        self.RemoveSubPart(self.o8);
      if (self.o9 > 0)
        self.RemoveSubPart(self.o9);
      if (self.o10 > 0)
        self.RemoveSubPart(self.o10);
      if (self.o11 > 0)
        self.RemoveSubPart(self.o11);
      if (self.o12 > 0)
        self.RemoveSubPart(self.o12);
      if (self.o13 > 0)
        self.RemoveSubPart(self.o13);
      if (self.o14 > 0)
        self.RemoveSubPart(self.o14);
      if (self.o15 > 0)
        self.RemoveSubPart(self.o15);
      if (self.o16 > 0)
        self.RemoveSubPart(self.o16);
      if (self.o17 > 0)
        self.RemoveSubPart(self.o17);
      if (self.o18 > 0)
        self.RemoveSubPart(self.o18);
      if (self.o19 > 0)
        self.RemoveSubPart(self.o19);
      if (self.h1 > 0)
        self.RemoveSubPart(self.h1);
      if (self.h2 > 0)
        self.RemoveSubPart(self.h2);
      if (self.h3 > 0)
        self.RemoveSubPart(self.h3);
      if (self.h4 > 0)
        self.RemoveSubPart(self.h4);
      if (self.h5 > 0)
        self.RemoveSubPart(self.h5);
      if (self.h6 > 0)
        self.RemoveSubPart(self.h6);
      if (self.h7 > 0)
        self.RemoveSubPart(self.h7);
      if (self.h8 > 0)
        self.RemoveSubPart(self.h8);
      if (self.h9 > 0)
        self.RemoveSubPart(self.h9);
      if (self.h10 > 0)
        self.RemoveSubPart(self.h10);
      if (self.h11 > 0)
        self.RemoveSubPart(self.h11);
      if (self.h12 > 0)
        self.RemoveSubPart(self.h12);
      if (self.z1 > 0)
        self.RemoveSubPart(self.z1);
      if (self.z2 > 0)
        self.RemoveSubPart(self.z2);
      if (self.w1 > 0)
        self.RemoveSubPart(self.w1);
      if (self.w2 > 0)
        self.RemoveSubPart(self.w2);
      if (self.w3 > 0)
        self.RemoveSubPart(self.w3);
      if (self.w4 > 0)
        self.RemoveSubPart(self.w4);
      if (self.w5 > 0)
        self.RemoveSubPart(self.w5);
      if (self.w6 > 0)
        self.RemoveSubPart(self.w6);
      if (self.w7 > 0)
        self.RemoveSubPart(self.w7);
      if (self.w8 > 0)
        self.RemoveSubPart(self.w8);
      if (self.w9 > 0)
        self.RemoveSubPart(self.w9);
      if (self.w10 > 0)
        self.RemoveSubPart(self.w10);
      if (self.w21 > 0)
        self.RemoveSubPart(self.w21);
      if (self.w11 > 0)
        self.RemoveSubPart(self.w11);
      if (self.w12 > 0)
        self.RemoveSubPart(self.w12);
      if (self.w13 > 0)
        self.RemoveSubPart(self.w13);
      if (self.w14 > 0)
        self.RemoveSubPart(self.w14);
      if (self.w15 > 0)
        self.RemoveSubPart(self.w15);
      if (self.w16 > 0)
        self.RemoveSubPart(self.w16);
      if (self.w17 > 0)
        self.RemoveSubPart(self.w17);
      if (self.r1 > 0)
        self.RemoveSubPart(self.r1);
      if (self.r2 > 0)
        self.RemoveSubPart(self.r2);
      if (self.r3 > 0)
        self.RemoveSubPart(self.r3);
      if (self.r4 > 0)
        self.RemoveSubPart(self.r4);
      if (self.r5 > 0)
        self.RemoveSubPart(self.r5);
      if (self.r6 > 0)
        self.RemoveSubPart(self.r6);
      if (self.r7 > 0)
        self.RemoveSubPart(self.r7);
      if (self.r8 > 0)
        self.RemoveSubPart(self.r8);
      if (self.r9 > 0)
        self.RemoveSubPart(self.r9);
      if (self.r10 > 0)
        self.RemoveSubPart(self.r10);
      if (self.r21 > 0)
        self.RemoveSubPart(self.r21);
      if (self.r11 > 0)
        self.RemoveSubPart(self.r11);
      if (self.r12 > 0)
        self.RemoveSubPart(self.r12);
      if (self.r13 > 0)
        self.RemoveSubPart(self.r13);
      if (self.r14 > 0)
        self.RemoveSubPart(self.r14);
      if (self.r15 > 0)
        self.RemoveSubPart(self.r15);
      if (self.r16 > 0)
        self.RemoveSubPart(self.r16);
      if (self.r17 > 0)
        self.RemoveSubPart(self.r17);
      if (self.r26 > 0)
        self.RemoveSubPart(self.r26);
      if (self.r27 > 0)
        self.RemoveSubPart(self.r27);
      if (self.r28 > 0)
        self.RemoveSubPart(self.r28);
      if (self.tab1 > 0)
        self.RemoveSubPart(self.tab1);
      if (self.tab2 > 0)
        self.RemoveSubPart(self.tab2);
      if (self.BStartGameID > 0)
        self.RemoveSubPart(self.BStartGameID);
      if (self.cancelID > 0)
        self.RemoveSubPart(self.cancelID);
      if (self.TempText > 0)
        self.RemoveSubPart(self.TempText);
      if (self.game.ModIntroType == 0)
        self.NewBackGroundAndClearAll(1024, 768, self.game.BACKGROUND2MARC);
      else
        self.NewBackGroundAndClearAll(1024, 768, self.game.BACKGROUND1MARC);
      Graphics Expression = Graphics.FromImage((Image) self.OwnBitmap);
      Expression.SmoothingMode = SmoothingMode.AntiAlias;
      Expression.TextRenderingHint = TextRenderingHint.AntiAlias;
      Expression.TextContrast = 1;
      let mut tsubpart1: SubPartClass =  new ATTextPartClass("Make Random Scenario Gold", self.game.VicFont8, 810, 35, true, tBlackBack: true);
      self.TempText = self.AddSubPart( tsubpart1, 100, 19, 850, 35, 0);
      let mut tsubpart2: SubPartClass =  new TextButtonPartClass("Basic settings", 150, tBackbitmap: ( self.OwnBitmap), bbx: 300, bby: 700, tred: (self.tabmode == 0));
      self.tab1 = self.AddSubPart( tsubpart2, 300, 700, 150, 35, 1);
      let mut tsubpart3: SubPartClass =  new TextButtonPartClass("Extra settings", 150, tBackbitmap: ( self.OwnBitmap), bbx: 500, bby: 700, tred: (self.tabmode == 1));
      self.tab2 = self.AddSubPart( tsubpart3, 500, 700, 150, 35, 1);
      DrawMod.DrawBlock( Expression, 35, 80, 935, 590,  self.game.VicColor4.R,  self.game.VicColor4.G,  self.game.VicColor4.B,  Math.Round( self.game.VicColor4.A / 2.0));
      DrawMod.DrawFrame( self.OwnBitmap,  self.BackBitmap,  Expression, 35, 80, 935, 590, -1, -1);
      SubPartClass tsubpart4;
      if (self.tabmode == 0)
      {
        vicFont2: Font = self.game.VicFont2;
        if (Strings.InStr(self.game.EditObj.RandomSettingsFromMod, "noworldsize") > 0)
        {
          self.Sworldsize = 0;
        }
        else
        {
          DrawMod.DrawTextVic2( Expression, "WORLD SIZE", self.game.VicFont1, 100, 97, self.game.VicColor2, self.game.VicColor2Shade);
          let mut tsubpart5: SubPartClass =  new SteveRadioPartClass(0, self.Sworldsize == 0, tBackbitmap: ( self.OwnBitmap), bbx: 100, bby: 130);
          self.o1 = self.AddSubPart( tsubpart5, 100, 130, 35, 35, 1);
          let mut tsubpart6: SubPartClass =  new SteveRadioPartClass(0, self.Sworldsize == 1, tBackbitmap: ( self.OwnBitmap), bbx: 100, bby: 170);
          self.o2 = self.AddSubPart( tsubpart6, 100, 170, 35, 35, 1);
          let mut tsubpart7: SubPartClass =  new SteveRadioPartClass(0, self.Sworldsize == 2, tBackbitmap: ( self.OwnBitmap), bbx: 100, bby: 210);
          self.o3 = self.AddSubPart( tsubpart7, 100, 210, 35, 35, 1);
          let mut tsubpart8: SubPartClass =  new SteveRadioPartClass(0, self.Sworldsize == 3, tBackbitmap: ( self.OwnBitmap), bbx: 100, bby: 250);
          self.o4 = self.AddSubPart( tsubpart8, 100, 250, 35, 35, 1);
          let mut tsubpart9: SubPartClass =  new SteveRadioPartClass(0, self.Sworldsize == 4, tBackbitmap: ( self.OwnBitmap), bbx: 100, bby: 290);
          self.o5 = self.AddSubPart( tsubpart9, 100, 290, 35, 35, 1);
          let mut tsubpart10: SubPartClass =  new SteveRadioPartClass(0, self.Sworldsize == 5, tBackbitmap: ( self.OwnBitmap), bbx: 100, bby: 330);
          self.o6 = self.AddSubPart( tsubpart10, 100, 330, 35, 35, 1);
          DrawMod.DrawTextVic2( Expression, "SMALL", vicFont2, 150, 138, self.game.VicColor2, self.game.VicColor2Shade);
          DrawMod.DrawTextVic2( Expression, "MEDIUM", vicFont2, 150, 178, self.game.VicColor2, self.game.VicColor2Shade);
          DrawMod.DrawTextVic2( Expression, "LARGE", vicFont2, 150, 218, self.game.VicColor2, self.game.VicColor2Shade);
          DrawMod.DrawTextVic2( Expression, "X-LARGE", vicFont2, 150, 258, self.game.VicColor2, self.game.VicColor2Shade);
          DrawMod.DrawTextVic2( Expression, "XX-LARGE", vicFont2, 150, 298, self.game.VicColor2, self.game.VicColor2Shade);
          DrawMod.DrawTextVic2( Expression, "UNPLAYABLY LARGE", vicFont2, 150, 338, self.game.VicColor2, self.game.VicColor2Shade);
        }
        DrawMod.DrawTextVic2( Expression, "PLAYERS", self.game.VicFont1, 370, 97, self.game.VicColor2, self.game.VicColor2Shade);
        let mut num: i32 = 14;
        if (Strings.InStr(self.game.EditObj.RandomSettingsFromMod, "maxplayer02") > 0)
          num = 2;
        if (Strings.InStr(self.game.EditObj.RandomSettingsFromMod, "maxplayer03") > 0)
          num = 3;
        if (Strings.InStr(self.game.EditObj.RandomSettingsFromMod, "maxplayer04") > 0)
          num = 4;
        if (Strings.InStr(self.game.EditObj.RandomSettingsFromMod, "maxplayer05") > 0)
          num = 5;
        if (Strings.InStr(self.game.EditObj.RandomSettingsFromMod, "maxplayer06") > 0)
          num = 6;
        if (Strings.InStr(self.game.EditObj.RandomSettingsFromMod, "maxplayer07") > 0)
          num = 7;
        if (Strings.InStr(self.game.EditObj.RandomSettingsFromMod, "maxplayer08") > 0)
          num = 8;
        if (Strings.InStr(self.game.EditObj.RandomSettingsFromMod, "maxplayer09") > 0)
          num = 9;
        if (Strings.InStr(self.game.EditObj.RandomSettingsFromMod, "maxplayer10") > 0)
          num = 10;
        if (Strings.InStr(self.game.EditObj.RandomSettingsFromMod, "maxplayer11") > 0)
          num = 11;
        if (Strings.InStr(self.game.EditObj.RandomSettingsFromMod, "maxplayer12") > 0)
          num = 12;
        if (Strings.InStr(self.game.EditObj.RandomSettingsFromMod, "maxplayer13") > 0)
          num = 13;
        if (Strings.InStr(self.game.EditObj.RandomSettingsFromMod, "maxplayer14") > 0)
          num = 14;
        if (num >= 2)
        {
          let mut tsubpart11: SubPartClass =  new SteveRadioPartClass(0, self.Splayer == 0, tBackbitmap: ( self.OwnBitmap), bbx: 370, bby: 130);
          self.w1 = self.AddSubPart( tsubpart11, 370, 130, 35, 35, 1);
        }
        if (num >= 3)
        {
          let mut tsubpart12: SubPartClass =  new SteveRadioPartClass(0, self.Splayer == 1, tBackbitmap: ( self.OwnBitmap), bbx: 370, bby: 170);
          self.w2 = self.AddSubPart( tsubpart12, 370, 170, 35, 35, 1);
        }
        if (num >= 4)
        {
          let mut tsubpart13: SubPartClass =  new SteveRadioPartClass(0, self.Splayer == 2, tBackbitmap: ( self.OwnBitmap), bbx: 370, bby: 210);
          self.w3 = self.AddSubPart( tsubpart13, 370, 210, 35, 35, 1);
        }
        if (num >= 5)
        {
          let mut tsubpart14: SubPartClass =  new SteveRadioPartClass(0, self.Splayer == 3, tBackbitmap: ( self.OwnBitmap), bbx: 370, bby: 250);
          self.w4 = self.AddSubPart( tsubpart14, 370, 250, 35, 35, 1);
        }
        if (num >= 6)
        {
          let mut tsubpart15: SubPartClass =  new SteveRadioPartClass(0, self.Splayer == 4, tBackbitmap: ( self.OwnBitmap), bbx: 370, bby: 290);
          self.w5 = self.AddSubPart( tsubpart15, 370, 290, 35, 35, 1);
        }
        if (num >= 7)
        {
          let mut tsubpart16: SubPartClass =  new SteveRadioPartClass(0, self.Splayer == 5, tBackbitmap: ( self.OwnBitmap), bbx: 370, bby: 330);
          self.w6 = self.AddSubPart( tsubpart16, 370, 330, 35, 35, 1);
        }
        if (num >= 8)
        {
          let mut tsubpart17: SubPartClass =  new SteveRadioPartClass(0, self.Splayer == 6, tBackbitmap: ( self.OwnBitmap), bbx: 530, bby: 130);
          self.w11 = self.AddSubPart( tsubpart17, 530, 130, 35, 35, 1);
        }
        if (num >= 9)
        {
          let mut tsubpart18: SubPartClass =  new SteveRadioPartClass(0, self.Splayer == 7, tBackbitmap: ( self.OwnBitmap), bbx: 530, bby: 170);
          self.w12 = self.AddSubPart( tsubpart18, 530, 170, 35, 35, 1);
        }
        if (num >= 10)
        {
          let mut tsubpart19: SubPartClass =  new SteveRadioPartClass(0, self.Splayer == 8, tBackbitmap: ( self.OwnBitmap), bbx: 530, bby: 210);
          self.w13 = self.AddSubPart( tsubpart19, 530, 210, 35, 35, 1);
        }
        if (num >= 11)
        {
          let mut tsubpart20: SubPartClass =  new SteveRadioPartClass(0, self.Splayer == 9, tBackbitmap: ( self.OwnBitmap), bbx: 530, bby: 250);
          self.w14 = self.AddSubPart( tsubpart20, 530, 250, 35, 35, 1);
        }
        if (num >= 12)
        {
          let mut tsubpart21: SubPartClass =  new SteveRadioPartClass(0, self.Splayer == 10, tBackbitmap: ( self.OwnBitmap), bbx: 530, bby: 290);
          self.w15 = self.AddSubPart( tsubpart21, 530, 290, 35, 35, 1);
        }
        if (num >= 13)
        {
          let mut tsubpart22: SubPartClass =  new SteveRadioPartClass(0, self.Splayer == 11, tBackbitmap: ( self.OwnBitmap), bbx: 530, bby: 330);
          self.w16 = self.AddSubPart( tsubpart22, 530, 330, 35, 35, 1);
        }
        if (num >= 14)
        {
          let mut tsubpart23: SubPartClass =  new SteveRadioPartClass(0, self.Splayer == 12, tBackbitmap: ( self.OwnBitmap), bbx: 530, bby: 370);
          self.w17 = self.AddSubPart( tsubpart23, 530, 370, 35, 35, 1);
        }
        if (num >= 2)
          DrawMod.DrawTextVic2( Expression, "2 PLAYER", vicFont2, 420, 138, self.game.VicColor2, self.game.VicColor2Shade);
        if (num >= 3)
          DrawMod.DrawTextVic2( Expression, "3 PLAYER", vicFont2, 420, 178, self.game.VicColor2, self.game.VicColor2Shade);
        if (num >= 4)
          DrawMod.DrawTextVic2( Expression, "4 PLAYER", vicFont2, 420, 218, self.game.VicColor2, self.game.VicColor2Shade);
        if (num >= 5)
          DrawMod.DrawTextVic2( Expression, "5 PLAYER", vicFont2, 420, 258, self.game.VicColor2, self.game.VicColor2Shade);
        if (num >= 6)
          DrawMod.DrawTextVic2( Expression, "6 PLAYER", vicFont2, 420, 298, self.game.VicColor2, self.game.VicColor2Shade);
        if (num >= 7)
          DrawMod.DrawTextVic2( Expression, "7 PLAYER", vicFont2, 420, 338, self.game.VicColor2, self.game.VicColor2Shade);
        if (num >= 8)
          DrawMod.DrawTextVic2( Expression, "8 PLAYER", vicFont2, 580, 138, self.game.VicColor2, self.game.VicColor2Shade);
        if (num >= 9)
          DrawMod.DrawTextVic2( Expression, "9 PLAYER", vicFont2, 580, 178, self.game.VicColor2, self.game.VicColor2Shade);
        if (num >= 10)
          DrawMod.DrawTextVic2( Expression, "10 PLAYER", vicFont2, 580, 218, self.game.VicColor2, self.game.VicColor2Shade);
        if (num >= 11)
          DrawMod.DrawTextVic2( Expression, "11 PLAYER", vicFont2, 580, 258, self.game.VicColor2, self.game.VicColor2Shade);
        if (num >= 12)
          DrawMod.DrawTextVic2( Expression, "12 PLAYER", vicFont2, 580, 298, self.game.VicColor2, self.game.VicColor2Shade);
        if (num >= 13)
          DrawMod.DrawTextVic2( Expression, "13 PLAYER", vicFont2, 580, 338, self.game.VicColor2, self.game.VicColor2Shade);
        if (num >= 14)
          DrawMod.DrawTextVic2( Expression, "14 PLAYER", vicFont2, 580, 378, self.game.VicColor2, self.game.VicColor2Shade);
        if (Strings.InStr(self.game.EditObj.RandomSettingsFromMod, "noworldtype") > 0)
        {
          self.Swater = 0;
        }
        else
        {
          DrawMod.DrawTextVic2( Expression, "WORLD TYPE", self.game.VicFont1, 750, 97, self.game.VicColor2, self.game.VicColor2Shade);
          let mut tsubpart24: SubPartClass =  new SteveRadioPartClass(0, self.Swater == 0, tBackbitmap: ( self.OwnBitmap), bbx: 750, bby: 130);
          self.h1 = self.AddSubPart( tsubpart24, 750, 130, 35, 35, 1);
          let mut tsubpart25: SubPartClass =  new SteveRadioPartClass(0, self.Swater == 1, tBackbitmap: ( self.OwnBitmap), bbx: 750, bby: 170);
          self.h2 = self.AddSubPart( tsubpart25, 750, 170, 35, 35, 1);
          let mut tsubpart26: SubPartClass =  new SteveRadioPartClass(0, self.Swater == 2, tBackbitmap: ( self.OwnBitmap), bbx: 750, bby: 210);
          self.h3 = self.AddSubPart( tsubpart26, 750, 210, 35, 35, 1);
          let mut tsubpart27: SubPartClass =  new SteveRadioPartClass(0, self.Swater == 3, tBackbitmap: ( self.OwnBitmap), bbx: 750, bby: 250);
          self.h4 = self.AddSubPart( tsubpart27, 750, 250, 35, 35, 1);
          let mut tsubpart28: SubPartClass =  new SteveRadioPartClass(0, self.Swater == 4, tBackbitmap: ( self.OwnBitmap), bbx: 750, bby: 290);
          self.h5 = self.AddSubPart( tsubpart28, 750, 290, 35, 35, 1);
          let mut tsubpart29: SubPartClass =  new SteveRadioPartClass(0, self.Swater == 5, tBackbitmap: ( self.OwnBitmap), bbx: 750, bby: 330);
          self.h6 = self.AddSubPart( tsubpart29, 750, 330, 35, 35, 1);
          DrawMod.DrawTextVic2( Expression, "NO OCEANS", vicFont2, 800, 138, self.game.VicColor2, self.game.VicColor2Shade);
          DrawMod.DrawTextVic2( Expression, "LAND WORLD", vicFont2, 800, 178, self.game.VicColor2, self.game.VicColor2Shade);
          DrawMod.DrawTextVic2( Expression, "NORMAL WORLD", vicFont2, 800, 218, self.game.VicColor2, self.game.VicColor2Shade);
          DrawMod.DrawTextVic2( Expression, "BIG OCEANS", vicFont2, 800, 258, self.game.VicColor2, self.game.VicColor2Shade);
          DrawMod.DrawTextVic2( Expression, "OCEANIA", vicFont2, 800, 298, self.game.VicColor2, self.game.VicColor2Shade);
          DrawMod.DrawTextVic2( Expression, "WATER WORLD", vicFont2, 800, 338, self.game.VicColor2, self.game.VicColor2Shade);
        }
        DrawMod.DrawTextVic2( Expression, "OPTIONS", self.game.VicFont1, 100, 417, self.game.VicColor2, self.game.VicColor2Shade);
        if (Strings.InStr(self.game.EditObj.RandomSettingsFromMod, "nomaploop") <= 0)
        {
          DrawMod.DrawTextVic2( Expression, "MAP LOOP", vicFont2, 150, 458, self.game.VicColor2, self.game.VicColor2Shade);
          let mut tsubpart30: SubPartClass =  new SteveRadioPartClass(0, self.domaploop == 1, tBackbitmap: ( self.OwnBitmap), bbx: 100, bby: 450);
          self.o7 = self.AddSubPart( tsubpart30, 100, 450, 35, 35, 1);
        }
        if (Strings.InStr(self.game.EditObj.RandomSettingsFromMod, "nomirrorish") > 0)
        {
          self.domirror = 0;
        }
        else
        {
          DrawMod.DrawTextVic2( Expression, "MIRRORISH", vicFont2, 150, 498, self.game.VicColor2, self.game.VicColor2Shade);
          let mut tsubpart31: SubPartClass =  new SteveRadioPartClass(0, self.domirror == 1, tBackbitmap: ( self.OwnBitmap), bbx: 100, bby: 490);
          self.o8 = self.AddSubPart( tsubpart31, 100, 490, 35, 35, 1);
        }
        if (Strings.InStr(self.game.EditObj.RandomSettingsFromMod, "noshrowd") > 0)
        {
          self.doshrowd = 0;
        }
        else
        {
          DrawMod.DrawTextVic2( Expression, "SHROUD", vicFont2, 150, 538, self.game.VicColor2, self.game.VicColor2Shade);
          let mut tsubpart32: SubPartClass =  new SteveRadioPartClass(0, self.doshrowd == 1, tBackbitmap: ( self.OwnBitmap), bbx: 100, bby: 530);
          self.o9 = self.AddSubPart( tsubpart32, 100, 530, 35, 35, 1);
        }
        if (Strings.InStr(self.game.EditObj.RandomSettingsFromMod, "no1townstart") > 0)
        {
          self.dosinglestart = 0;
        }
        else
        {
          DrawMod.DrawTextVic2( Expression, "1 TOWN START", vicFont2, 150, 578, self.game.VicColor2, self.game.VicColor2Shade);
          let mut tsubpart33: SubPartClass =  new SteveRadioPartClass(0, self.dosinglestart == 1, tBackbitmap: ( self.OwnBitmap), bbx: 100, bby: 570);
          self.o10 = self.AddSubPart( tsubpart33, 100, 570, 35, 35, 1);
        }
        if (Strings.InStr(self.game.EditObj.RandomSettingsFromMod, "nocostlyresearch") > 0)
        {
          self.opt11v = 100;
        }
        else
        {
          DrawMod.DrawTextVic2( Expression, "COSTLY RESEARCH", vicFont2, 150, 618, self.game.VicColor2, self.game.VicColor2Shade);
          let mut tsubpart34: SubPartClass =  new SteveRadioPartClass(0, self.opt11v == 300, tBackbitmap: ( self.OwnBitmap), bbx: 100, bby: 610);
          self.o19 = self.AddSubPart( tsubpart34, 100, 610, 35, 35, 1);
        }
        if (Strings.InStr(self.game.EditObj.RandomSettingsFromMod, "nostoneage") > 0)
        {
          self.dofirsttech = 1;
        }
        else
        {
          DrawMod.DrawTextVic2( Expression, "STONE AGE", vicFont2, 350, 458, self.game.VicColor2, self.game.VicColor2Shade);
          let mut tsubpart35: SubPartClass =  new SteveRadioPartClass(0, self.dofirsttech == 0, tBackbitmap: ( self.OwnBitmap), bbx: 300, bby: 450);
          self.o12 = self.AddSubPart( tsubpart35, 300, 450, 35, 35, 1);
        }
        if (Strings.InStr(self.game.EditObj.RandomSettingsFromMod, "nocrates") > 0)
        {
          self.Scrate = 1;
        }
        else
        {
          DrawMod.DrawTextVic2( Expression, "CRATES", vicFont2, 350, 488, self.game.VicColor2, self.game.VicColor2Shade);
          DrawMod.DrawTextVic2( Expression, "HIDDEN AI", vicFont2, 350, 508, self.game.VicColor2, self.game.VicColor2Shade);
          let mut tsubpart36: SubPartClass =  new SteveRadioPartClass(0, self.Scrate == 0, tBackbitmap: ( self.OwnBitmap), bbx: 300, bby: 490);
          self.o13 = self.AddSubPart( tsubpart36, 300, 490, 35, 35, 1);
        }
        if (Strings.InStr(self.game.EditObj.RandomSettingsFromMod, "nohiddenstats") > 0)
        {
          self.dostats = 1;
        }
        else
        {
          DrawMod.DrawTextVic2( Expression, "HIDDEN STATS", vicFont2, 350, 538, self.game.VicColor2, self.game.VicColor2Shade);
          let mut tsubpart37: SubPartClass =  new SteveRadioPartClass(0, self.dostats == 0, tBackbitmap: ( self.OwnBitmap), bbx: 300, bby: 530);
          self.o14 = self.AddSubPart( tsubpart37, 300, 530, 35, 35, 1);
        }
        if (Strings.InStr(self.game.EditObj.RandomSettingsFromMod, "noaiallied") > 0)
        {
          self.doallied = 0;
        }
        else
        {
          DrawMod.DrawTextVic2( Expression, "AI ALLIED", vicFont2, 350, 578, self.game.VicColor2, self.game.VicColor2Shade);
          let mut tsubpart38: SubPartClass =  new SteveRadioPartClass(0, self.doallied == 1, tBackbitmap: ( self.OwnBitmap), bbx: 300, bby: 570);
          self.o15 = self.AddSubPart( tsubpart38, 300, 570, 35, 35, 1);
        }
        if (Strings.InStr(self.game.EditObj.RandomSettingsFromMod, "nouseresources") > 0)
        {
          self.Srawuse = 0;
        }
        else
        {
          DrawMod.DrawTextVic2( Expression, "USE RESOURCES", vicFont2, 350, 618, self.game.VicColor2, self.game.VicColor2Shade);
          let mut tsubpart39: SubPartClass =  new SteveRadioPartClass(0, self.Srawuse == 1, tBackbitmap: ( self.OwnBitmap), bbx: 300, bby: 610);
          self.z1 = self.AddSubPart( tsubpart39, 300, 610, 35, 35, 1);
        }
        if (Strings.InStr(self.game.EditObj.RandomSettingsFromMod, "noroads") > 0)
        {
          self.opt9v = 0;
        }
        else
        {
          DrawMod.DrawTextVic2( Expression, "NO ROADS", vicFont2, 550, 458, self.game.VicColor2, self.game.VicColor2Shade);
          let mut tsubpart40: SubPartClass =  new SteveRadioPartClass(0, self.opt9v == 0, tBackbitmap: ( self.OwnBitmap), bbx: 500, bby: 450);
          self.o11 = self.AddSubPart( tsubpart40, 500, 450, 35, 35, 1);
        }
        if (Strings.InStr(self.game.EditObj.RandomSettingsFromMod, "nowildlands") > 0)
        {
          self.Spop = 0;
        }
        else
        {
          DrawMod.DrawTextVic2( Expression, "WILD LAND", vicFont2, 550, 498, self.game.VicColor2, self.game.VicColor2Shade);
          let mut tsubpart41: SubPartClass =  new SteveRadioPartClass(0, self.Spop == 1, tBackbitmap: ( self.OwnBitmap), bbx: 500, bby: 490);
          self.o16 = self.AddSubPart( tsubpart41, 500, 490, 35, 35, 1);
        }
        if (Strings.InStr(self.game.EditObj.RandomSettingsFromMod, "nodepletedlands") > 0)
        {
          self.Sraw = 0;
        }
        else
        {
          DrawMod.DrawTextVic2( Expression, "DEPLETED LAND", vicFont2, 550, 538, self.game.VicColor2, self.game.VicColor2Shade);
          let mut tsubpart42: SubPartClass =  new SteveRadioPartClass(0, self.Sraw == 1, tBackbitmap: ( self.OwnBitmap), bbx: 500, bby: 530);
          self.o17 = self.AddSubPart( tsubpart42, 500, 530, 35, 35, 1);
        }
        if (Strings.InStr(self.game.EditObj.RandomSettingsFromMod, "nodesertedlands") > 0)
        {
          self.Spop = 0;
        }
        else
        {
          DrawMod.DrawTextVic2( Expression, "DESERTED LANDS", vicFont2, 550, 578, self.game.VicColor2, self.game.VicColor2Shade);
          let mut tsubpart43: SubPartClass =  new SteveRadioPartClass(0, self.Spop == 2, tBackbitmap: ( self.OwnBitmap), bbx: 500, bby: 570);
          self.o18 = self.AddSubPart( tsubpart43, 500, 570, 35, 35, 1);
        }
        if (Strings.InStr(self.game.EditObj.RandomSettingsFromMod, "nochangemaster") <= 0)
        {
          tsubpart4 =  new SteveRadioPartClass(0, false, tBackbitmap: ( self.OwnBitmap), bbx: 500, bby: 610);
          self.z2 = self.AddSubPart( tsubpart4, 500, 610, 35, 35, 1);
          DrawMod.DrawTextVic2( Expression, Strings.Left(self.domasterfile, Math.Min(Strings.Len(self.domasterfile), 14)), vicFont2, 550, 618, self.game.VicColor2, self.game.VicColor2Shade);
        }
        if (Strings.InStr(self.game.EditObj.RandomSettingsFromMod, "noweather") > 0)
        {
          self.Sclimate = 0;
        }
        else
        {
          DrawMod.DrawTextVic2( Expression, "CLIMATE", self.game.VicFont1, 750, 417, self.game.VicColor2, self.game.VicColor2Shade);
          tsubpart4 =  new SteveRadioPartClass(0, self.Sclimate == 0, tBackbitmap: ( self.OwnBitmap), bbx: 750, bby: 450);
          self.w7 = self.AddSubPart( tsubpart4, 750, 450, 35, 35, 1);
          tsubpart4 =  new SteveRadioPartClass(0, self.Sclimate == 1, tBackbitmap: ( self.OwnBitmap), bbx: 750, bby: 490);
          self.w8 = self.AddSubPart( tsubpart4, 750, 490, 35, 35, 1);
          tsubpart4 =  new SteveRadioPartClass(0, self.Sclimate == 2, tBackbitmap: ( self.OwnBitmap), bbx: 750, bby: 530);
          self.w9 = self.AddSubPart( tsubpart4, 750, 530, 35, 35, 1);
          tsubpart4 =  new SteveRadioPartClass(0, self.Sclimate == 3, tBackbitmap: ( self.OwnBitmap), bbx: 750, bby: 570);
          self.w10 = self.AddSubPart( tsubpart4, 750, 570, 35, 35, 1);
          tsubpart4 =  new SteveRadioPartClass(0, self.Sclimate == 4, tBackbitmap: ( self.OwnBitmap), bbx: 750, bby: 610);
          self.w21 = self.AddSubPart( tsubpart4, 750, 610, 35, 35, 1);
          DrawMod.DrawTextVic2( Expression, "NO CLIMATE", vicFont2, 800, 460, self.game.VicColor2, self.game.VicColor2Shade);
          DrawMod.DrawTextVic2( Expression, "FULL RANGE", vicFont2, 800, 500, self.game.VicColor2, self.game.VicColor2Shade);
          DrawMod.DrawTextVic2( Expression, "ARCTIC-TEMPERATE", vicFont2, 800, 540, self.game.VicColor2, self.game.VicColor2Shade);
          DrawMod.DrawTextVic2( Expression, "SUBTROPIC-TROPIC", vicFont2, 800, 580, self.game.VicColor2, self.game.VicColor2Shade);
          DrawMod.DrawTextVic2( Expression, "TEMPERATE", vicFont2, 800, 620, self.game.VicColor2, self.game.VicColor2Shade);
        }
      }
      else if (self.tabmode == 1)
      {
        vicFont2: Font = self.game.VicFont2;
        if (Strings.InStr(self.game.EditObj.RandomSettingsFromMod, "nocontinentalsize") > 0)
        {
          self.optr1 = 0;
        }
        else
        {
          DrawMod.DrawTextVic2( Expression, "CONTINENTAL SIZE", self.game.VicFont1, 100, 97, self.game.VicColor2, self.game.VicColor2Shade);
          let mut tsubpart44: SubPartClass =  new SteveRadioPartClass(0, self.optr1 == -2, tBackbitmap: ( self.OwnBitmap), bbx: 100, bby: 130);
          self.r1 = self.AddSubPart( tsubpart44, 100, 130, 35, 35, 1);
          let mut tsubpart45: SubPartClass =  new SteveRadioPartClass(0, self.optr1 == -1, tBackbitmap: ( self.OwnBitmap), bbx: 100, bby: 170);
          self.r2 = self.AddSubPart( tsubpart45, 100, 170, 35, 35, 1);
          let mut tsubpart46: SubPartClass =  new SteveRadioPartClass(0, self.optr1 == 0, tBackbitmap: ( self.OwnBitmap), bbx: 100, bby: 210);
          self.r3 = self.AddSubPart( tsubpart46, 100, 210, 35, 35, 1);
          let mut tsubpart47: SubPartClass =  new SteveRadioPartClass(0, self.optr1 == 1, tBackbitmap: ( self.OwnBitmap), bbx: 100, bby: 250);
          self.r4 = self.AddSubPart( tsubpart47, 100, 250, 35, 35, 1);
          let mut tsubpart48: SubPartClass =  new SteveRadioPartClass(0, self.optr1 == 2, tBackbitmap: ( self.OwnBitmap), bbx: 100, bby: 290);
          self.r5 = self.AddSubPart( tsubpart48, 100, 290, 35, 35, 1);
          DrawMod.DrawTextVic2( Expression, "TINY", vicFont2, 150, 138, self.game.VicColor2, self.game.VicColor2Shade);
          DrawMod.DrawTextVic2( Expression, "SMALL", vicFont2, 150, 178, self.game.VicColor2, self.game.VicColor2Shade);
          DrawMod.DrawTextVic2( Expression, "NORMAL", vicFont2, 150, 218, self.game.VicColor2, self.game.VicColor2Shade);
          DrawMod.DrawTextVic2( Expression, "BIG", vicFont2, 150, 258, self.game.VicColor2, self.game.VicColor2Shade);
          DrawMod.DrawTextVic2( Expression, "PANGEAIC", vicFont2, 150, 298, self.game.VicColor2, self.game.VicColor2Shade);
        }
        let mut num: i32 = 50;
        if (Strings.InStr(self.game.EditObj.RandomSettingsFromMod, "nohumidity") > 0)
        {
          self.optr2 = 0;
        }
        else
        {
          DrawMod.DrawTextVic2( Expression, "HUMIDITY", self.game.VicFont1, 370 + num, 97, self.game.VicColor2, self.game.VicColor2Shade);
          let mut tsubpart49: SubPartClass =  new SteveRadioPartClass(0, self.optr2 == -2, tBackbitmap: ( self.OwnBitmap), bbx: (370 + num), bby: 130);
          self.r6 = self.AddSubPart( tsubpart49, 370 + num, 130, 35, 35, 1);
          let mut tsubpart50: SubPartClass =  new SteveRadioPartClass(0, self.optr2 == -1, tBackbitmap: ( self.OwnBitmap), bbx: (370 + num), bby: 170);
          self.r7 = self.AddSubPart( tsubpart50, 370 + num, 170, 35, 35, 1);
          let mut tsubpart51: SubPartClass =  new SteveRadioPartClass(0, self.optr2 == 0, tBackbitmap: ( self.OwnBitmap), bbx: (370 + num), bby: 210);
          self.r8 = self.AddSubPart( tsubpart51, 370 + num, 210, 35, 35, 1);
          let mut tsubpart52: SubPartClass =  new SteveRadioPartClass(0, self.optr2 == 1, tBackbitmap: ( self.OwnBitmap), bbx: (370 + num), bby: 250);
          self.r9 = self.AddSubPart( tsubpart52, 370 + num, 250, 35, 35, 1);
          let mut tsubpart53: SubPartClass =  new SteveRadioPartClass(0, self.optr2 == 2, tBackbitmap: ( self.OwnBitmap), bbx: (370 + num), bby: 290);
          self.r10 = self.AddSubPart( tsubpart53, 370 + num, 290, 35, 35, 1);
          DrawMod.DrawTextVic2( Expression, "ARID WORLD", vicFont2, 420 + num, 138, self.game.VicColor2, self.game.VicColor2Shade);
          DrawMod.DrawTextVic2( Expression, "DRY WORLD", vicFont2, 420 + num, 178, self.game.VicColor2, self.game.VicColor2Shade);
          DrawMod.DrawTextVic2( Expression, "NORMAL", vicFont2, 420 + num, 218, self.game.VicColor2, self.game.VicColor2Shade);
          DrawMod.DrawTextVic2( Expression, "WET WORLD", vicFont2, 420 + num, 258, self.game.VicColor2, self.game.VicColor2Shade);
          DrawMod.DrawTextVic2( Expression, "RAIN WORLD", vicFont2, 420 + num, 298, self.game.VicColor2, self.game.VicColor2Shade);
        }
        if (Strings.InStr(self.game.EditObj.RandomSettingsFromMod, "nogeologicalage") > 0)
        {
          self.optr3 = 0;
        }
        else
        {
          DrawMod.DrawTextVic2( Expression, "GEOLOGICAL AGE", self.game.VicFont1, 750, 97, self.game.VicColor2, self.game.VicColor2Shade);
          let mut tsubpart54: SubPartClass =  new SteveRadioPartClass(0, self.optr3 == -2, tBackbitmap: ( self.OwnBitmap), bbx: 750, bby: 130);
          self.r11 = self.AddSubPart( tsubpart54, 750, 130, 35, 35, 1);
          let mut tsubpart55: SubPartClass =  new SteveRadioPartClass(0, self.optr3 == -1, tBackbitmap: ( self.OwnBitmap), bbx: 750, bby: 170);
          self.r12 = self.AddSubPart( tsubpart55, 750, 170, 35, 35, 1);
          let mut tsubpart56: SubPartClass =  new SteveRadioPartClass(0, self.optr3 == 0, tBackbitmap: ( self.OwnBitmap), bbx: 750, bby: 210);
          self.r13 = self.AddSubPart( tsubpart56, 750, 210, 35, 35, 1);
          let mut tsubpart57: SubPartClass =  new SteveRadioPartClass(0, self.optr3 == 1, tBackbitmap: ( self.OwnBitmap), bbx: 750, bby: 250);
          self.r14 = self.AddSubPart( tsubpart57, 750, 250, 35, 35, 1);
          let mut tsubpart58: SubPartClass =  new SteveRadioPartClass(0, self.optr3 == 2, tBackbitmap: ( self.OwnBitmap), bbx: 750, bby: 290);
          self.r15 = self.AddSubPart( tsubpart58, 750, 290, 35, 35, 1);
          DrawMod.DrawTextVic2( Expression, "VERY YOUNG", vicFont2, 800, 138, self.game.VicColor2, self.game.VicColor2Shade);
          DrawMod.DrawTextVic2( Expression, "YOUNG", vicFont2, 800, 178, self.game.VicColor2, self.game.VicColor2Shade);
          DrawMod.DrawTextVic2( Expression, "NORMAL", vicFont2, 800, 218, self.game.VicColor2, self.game.VicColor2Shade);
          DrawMod.DrawTextVic2( Expression, "OLD", vicFont2, 800, 258, self.game.VicColor2, self.game.VicColor2Shade);
          DrawMod.DrawTextVic2( Expression, "ANCIENT", vicFont2, 800, 298, self.game.VicColor2, self.game.VicColor2Shade);
        }
        DrawMod.DrawTextVic2( Expression, "EXTRA OPTIONS", self.game.VicFont1, 100, 417, self.game.VicColor2, self.game.VicColor2Shade);
        if (Strings.InStr(self.game.EditObj.RandomSettingsFromMod, "nolimitedroads") > 0)
        {
          self.optr4 = 0;
        }
        else
        {
          let mut tsubpart59: SubPartClass =  new SteveRadioPartClass(0, self.optr4 == 1, tBackbitmap: ( self.OwnBitmap), bbx: 100, bby: 450);
          self.r16 = self.AddSubPart( tsubpart59, 100, 450, 35, 35, 1);
          DrawMod.DrawTextVic2( Expression, "LIMITED INITIAL ROADS", vicFont2, 150, 458, self.game.VicColor2, self.game.VicColor2Shade);
        }
        if (Strings.InStr(self.game.EditObj.RandomSettingsFromMod, "nonaturalcoastlines") > 0)
        {
          self.optr5 = 0;
        }
        else
        {
          let mut tsubpart60: SubPartClass =  new SteveRadioPartClass(0, self.optr5 == 1, tBackbitmap: ( self.OwnBitmap), bbx: 100, bby: 490);
          self.r17 = self.AddSubPart( tsubpart60, 100, 490, 35, 35, 1);
          DrawMod.DrawTextVic2( Expression, "NATURAL COASTLINES", vicFont2, 150, 498, self.game.VicColor2, self.game.VicColor2Shade);
        }
        if (Strings.InStr(self.game.EditObj.RandomSettingsFromMod, "nonofarm") > 0)
        {
          self.optr6 = 0;
        }
        else
        {
          let mut tsubpart61: SubPartClass =  new SteveRadioPartClass(0, self.optr6 == 1, tBackbitmap: ( self.OwnBitmap), bbx: 100, bby: 530);
          self.r26 = self.AddSubPart( tsubpart61, 100, 530, 35, 35, 1);
          DrawMod.DrawTextVic2( Expression, "NO FARMLANDS", vicFont2, 150, 538, self.game.VicColor2, self.game.VicColor2Shade);
        }
        if (Strings.InStr(self.game.EditObj.RandomSettingsFromMod, "nonosuburb") > 0)
        {
          self.optr7 = 0;
        }
        else
        {
          let mut tsubpart62: SubPartClass =  new SteveRadioPartClass(0, self.optr7 == 1, tBackbitmap: ( self.OwnBitmap), bbx: 100, bby: 570);
          self.r27 = self.AddSubPart( tsubpart62, 100, 570, 35, 35, 1);
          DrawMod.DrawTextVic2( Expression, "NO SUBURBS", vicFont2, 150, 578, self.game.VicColor2, self.game.VicColor2Shade);
        }
        if (Strings.InStr(self.game.EditObj.RandomSettingsFromMod, "nohigherprodcost") > 0)
        {
          self.optr8 = 0;
        }
        else
        {
          let mut tsubpart63: SubPartClass =  new SteveRadioPartClass(0, self.optr8 == 1, tBackbitmap: ( self.OwnBitmap), bbx: 100, bby: 610);
          self.r28 = self.AddSubPart( tsubpart63, 100, 610, 35, 35, 1);
          DrawMod.DrawTextVic2( Expression, "HIGHER PROD. COST", vicFont2, 150, 618, self.game.VicColor2, self.game.VicColor2Shade);
        }
      }
      tsubpart4 =  new TextButtonPartClass("Make", 100, tBackbitmap: ( self.OwnBitmap), bbx: 880, bby: 700);
      self.BStartGameID = self.AddSubPart( tsubpart4, 880, 700, 100, 35, 1);
      tsubpart4 =  new SteveButtonPartClass(self.game.BACKBUTTON, tBackbitmap: ( self.OwnBitmap), bbx: 30, bby: 700);
      self.cancelID = self.AddSubPart( tsubpart4, 30, 700, 35, 35, 1);
      if (Information.IsNothing( Expression))
        return;
      Expression.Dispose();
      Expression = (Graphics) null;
    }

    pub fn DoStuff()
    {
      if (self.game.EditObj.ShortRandomScreen)
      {
        self.DoStuffShort();
      }
      else
      {
        if (self.TempText > 0)
          self.RemoveSubPart(self.TempText);
        if (self.TempText2 > 0)
          self.RemoveSubPart(self.TempText2);
        if (self.BStartGameID > 0)
          self.RemoveSubPart(self.BStartGameID);
        if (self.BLoadGameID > 0)
          self.RemoveSubPart(self.BLoadGameID);
        if (self.BEditorID > 0)
          self.RemoveSubPart(self.BEditorID);
        if (self.RegimeListId > 0)
          self.RemoveSubPart(self.RegimeListId);
        if (self.allied > 0)
          self.RemoveSubPart(self.allied);
        if (self.alliedtext > 0)
          self.RemoveSubPart(self.alliedtext);
        if (self.optimize > 0)
          self.RemoveSubPart(self.optimize);
        if (self.optimizetext > 0)
          self.RemoveSubPart(self.optimizetext);
        if (self.opt1 > 0)
          self.RemoveSubPart(self.opt1);
        if (self.txt1 > 0)
          self.RemoveSubPart(self.txt1);
        if (self.opt2 > 0)
          self.RemoveSubPart(self.opt2);
        if (self.txt2 > 0)
          self.RemoveSubPart(self.txt2);
        if (self.opt3 > 0)
          self.RemoveSubPart(self.opt3);
        if (self.txt3 > 0)
          self.RemoveSubPart(self.txt3);
        if (self.opt4 > 0)
          self.RemoveSubPart(self.opt4);
        if (self.opt5 > 0)
          self.RemoveSubPart(self.opt5);
        if (self.opt6 > 0)
          self.RemoveSubPart(self.opt6);
        if (self.opt7 > 0)
          self.RemoveSubPart(self.opt7);
        if (self.opt8 > 0)
          self.RemoveSubPart(self.opt8);
        if (self.opt9 > 0)
          self.RemoveSubPart(self.opt9);
        if (self.opt10 > 0)
          self.RemoveSubPart(self.opt10);
        if (self.opt11 > 0)
          self.RemoveSubPart(self.opt11);
        if (self.opt12 > 0)
          self.RemoveSubPart(self.opt12);
        if (self.cancelID > 0)
          self.RemoveSubPart(self.cancelID);
        if (self.shrowd > 0)
          self.RemoveSubPart(self.shrowd);
        if (self.shrowdtext > 0)
          self.RemoveSubPart(self.shrowdtext);
        if (self.mirror > 0)
          self.RemoveSubPart(self.mirror);
        if (self.mirrortext > 0)
          self.RemoveSubPart(self.mirrortext);
        if (self.blockcenter > 0)
          self.RemoveSubPart(self.blockcenter);
        if (self.blockcentertext > 0)
          self.RemoveSubPart(self.blockcentertext);
        if (self.firsttech > 0)
          self.RemoveSubPart(self.firsttech);
        if (self.firsttechtext > 0)
          self.RemoveSubPart(self.firsttechtext);
        if (self.masterfile > 0)
          self.RemoveSubPart(self.masterfile);
        if (self.masterfiletext > 0)
          self.RemoveSubPart(self.masterfiletext);
        if (self.maploop > 0)
          self.RemoveSubPart(self.maploop);
        if (self.maplooptext > 0)
          self.RemoveSubPart(self.maplooptext);
        if (self.oldkingdom > 0)
          self.RemoveSubPart(self.oldkingdom);
        if (self.oldkingdomtext > 0)
          self.RemoveSubPart(self.oldkingdomtext);
        let mut tsubpart: SubPartClass =  TextPartClass::new("Make Random Scenario", Font::new("Arial Black", 19f, FontStyle.Regular, GraphicsUnit.Pixel), 450, 27, false, tBlackBack: true);
        self.TempText = self.AddSubPart( tsubpart, 200, 19, 150, 16, 0);
        tsubpart =  new NumberSliderSubPartClass3(self.game, "Width= ", " hex ", 400, 10, 100, self.opt1v, tbackbitmap: ( self.OwnBitmap), bbx: 200, bby: 100);
        self.opt1 = self.AddSubPart( tsubpart, 200, 100, 400, 22, 0);
        tsubpart =  new NumberSliderSubPartClass3(self.game, "Towns= ", "x ", 400, 0, 100, self.opt8v, tbackbitmap: ( self.OwnBitmap), bbx: 200, bby: 390);
        self.opt8 = self.AddSubPart( tsubpart, 200, 390, 400, 22, 0);
        tsubpart =  new NumberSliderSubPartClass3(self.game, "Height= ", " hex ", 400, 10, 100, self.opt2v, tbackbitmap: ( self.OwnBitmap), bbx: 200, bby: 140);
        self.opt2 = self.AddSubPart( tsubpart, 200, 140, 400, 22, 0);
        tsubpart =  new NumberSliderSubPartClass3(self.game, "Roads= level ", "", 400, 0, 5, self.opt9v, tbackbitmap: ( self.OwnBitmap), bbx: 200, bby: 430);
        self.opt9 = self.AddSubPart( tsubpart, 200, 430, 400, 22, 0);
        tsubpart =  new NumberSliderSubPartClass3(self.game, "Players= ", " regimes ", 400, 2, 10, self.opt3v, tbackbitmap: ( self.OwnBitmap), bbx: 200, bby: 180);
        self.opt3 = self.AddSubPart( tsubpart, 200, 180, 400, 22, 0);
        tsubpart =  new NumberSliderSubPartClass3(self.game, "City Size= ", "", 400, 1, 4, self.opt10v, tbackbitmap: ( self.OwnBitmap), bbx: 200, bby: 470);
        self.opt10 = self.AddSubPart( tsubpart, 200, 470, 400, 22, 0);
        tsubpart =  new NumberSliderSubPartClass3(self.game, "Land = ", "%", 400, 10, 100, self.opt4v, tbackbitmap: ( self.OwnBitmap), bbx: 200, bby: 220);
        self.opt4 = self.AddSubPart( tsubpart, 200, 220, 400, 22, 0);
        tsubpart =  new NumberSliderSubPartClass3(self.game, "Research mod= ", "% ", 400, 20, 500, self.opt11v, tbackbitmap: ( self.OwnBitmap), bbx: 200, bby: 510);
        self.opt11 = self.AddSubPart( tsubpart, 200, 510, 400, 22, 0);
        tsubpart =  new NumberSliderSubPartClass3(self.game, "Rivers = ", "% ", 400, 0, 100, self.opt5v, tbackbitmap: ( self.OwnBitmap), bbx: 200, bby: 260);
        self.opt5 = self.AddSubPart( tsubpart, 200, 260, 400, 22, 0);
        tsubpart =  new NumberSliderSubPartClass3(self.game, "Swamp= level", "", 400, 0, 100, self.opt12v, tbackbitmap: ( self.OwnBitmap), bbx: 200, bby: 550);
        self.opt12 = self.AddSubPart( tsubpart, 200, 550, 400, 22, 0);
        tsubpart =  new NumberSliderSubPartClass3(self.game, "Forests = level ", "", 400, 0, 100, self.opt6v, tbackbitmap: ( self.OwnBitmap), bbx: 200, bby: 300);
        self.opt6 = self.AddSubPart( tsubpart, 200, 300, 400, 22, 0);
        tsubpart =  new NumberSliderSubPartClass3(self.game, "Mountains = level ", "", 400, 0, 100, self.opt7v, tbackbitmap: ( self.OwnBitmap), bbx: 200, bby: 340);
        self.opt7 = self.AddSubPart( tsubpart, 200, 340, 400, 22, 0);
        if (self.dooptimize == 0)
        {
          tsubpart =  new SteveButtonPartClass(self.game.CANCELBALL, tBackbitmap: ( self.OwnBitmap), bbx: 700, bby: 260);
          self.optimize = self.AddSubPart( tsubpart, 700, 220, 35, 35, 1);
        }
        else
        {
          tsubpart =  new SteveButtonPartClass(self.game.OKBALL, tBackbitmap: ( self.OwnBitmap), bbx: 700, bby: 260);
          self.optimize = self.AddSubPart( tsubpart, 700, 220, 35, 35, 1);
        }
        tsubpart =  TextPartClass::new("Optimize for AI", Font::new("Times New Roman", 13f, FontStyle.Regular, GraphicsUnit.Pixel), 150, 16, false);
        self.optimizetext = self.AddSubPart( tsubpart, 750, 229, 150, 16, 0);
        if (self.dooldkingdom == 0)
        {
          tsubpart =  new SteveButtonPartClass(self.game.CANCELBALL, tBackbitmap: ( self.OwnBitmap), bbx: 700, bby: 260);
          self.oldkingdom = self.AddSubPart( tsubpart, 700, 260, 35, 35, 1);
        }
        else
        {
          tsubpart =  new SteveButtonPartClass(self.game.OKBALL, tBackbitmap: ( self.OwnBitmap), bbx: 700, bby: 260);
          self.oldkingdom = self.AddSubPart( tsubpart, 700, 260, 35, 35, 1);
        }
        tsubpart =  TextPartClass::new("People's Republic", Font::new("Times New Roman", 13f, FontStyle.Regular, GraphicsUnit.Pixel), 150, 16, false);
        self.oldkingdomtext = self.AddSubPart( tsubpart, 750, 269, 150, 16, 0);
        if (self.domaploop == 0)
        {
          tsubpart =  new SteveButtonPartClass(self.game.CANCELBALL, tBackbitmap: ( self.OwnBitmap), bbx: 700, bby: 300);
          self.maploop = self.AddSubPart( tsubpart, 700, 300, 35, 35, 1);
        }
        else
        {
          tsubpart =  new SteveButtonPartClass(self.game.OKBALL, tBackbitmap: ( self.OwnBitmap), bbx: 700, bby: 300);
          self.maploop = self.AddSubPart( tsubpart, 700, 300, 35, 35, 1);
        }
        tsubpart =  TextPartClass::new("Map Loop", Font::new("Times New Roman", 13f, FontStyle.Regular, GraphicsUnit.Pixel), 150, 16, false);
        self.maplooptext = self.AddSubPart( tsubpart, 750, 309, 150, 16, 0);
        if (self.doshrowd == 0)
        {
          tsubpart =  new SteveButtonPartClass(self.game.CANCELBALL, tBackbitmap: ( self.OwnBitmap), bbx: 700, bby: 340);
          self.shrowd = self.AddSubPart( tsubpart, 700, 340, 35, 35, 1);
        }
        else
        {
          tsubpart =  new SteveButtonPartClass(self.game.OKBALL, tBackbitmap: ( self.OwnBitmap), bbx: 700, bby: 340);
          self.shrowd = self.AddSubPart( tsubpart, 700, 340, 35, 35, 1);
        }
        tsubpart =  TextPartClass::new("Shrouded Map", Font::new("Times New Roman", 13f, FontStyle.Regular, GraphicsUnit.Pixel), 150, 16, false);
        self.shrowdtext = self.AddSubPart( tsubpart, 750, 349, 150, 16, 0);
        if (self.opt3v == 2)
        {
          if (self.domirror == 0)
          {
            tsubpart =  new SteveButtonPartClass(self.game.CANCELBALL, tBackbitmap: ( self.OwnBitmap), bbx: 700, bby: 380);
            self.mirror = self.AddSubPart( tsubpart, 700, 380, 35, 35, 1);
          }
          else
          {
            tsubpart =  new SteveButtonPartClass(self.game.OKBALL, tBackbitmap: ( self.OwnBitmap), bbx: 700, bby: 380);
            self.mirror = self.AddSubPart( tsubpart, 700, 380, 35, 35, 1);
          }
          tsubpart =  TextPartClass::new("Mirror Map", Font::new("Times New Roman", 13f, FontStyle.Regular, GraphicsUnit.Pixel), 150, 16, false);
          self.mirrortext = self.AddSubPart( tsubpart, 750, 389, 150, 16, 0);
        }
        else
          self.domirror = 0;
        if (self.doblockcenter == 0)
        {
          tsubpart =  new SteveButtonPartClass(self.game.CANCELBALL, tBackbitmap: ( self.OwnBitmap), bbx: 700, bby: 420);
          self.blockcenter = self.AddSubPart( tsubpart, 700, 420, 35, 35, 1);
        }
        else
        {
          tsubpart =  new SteveButtonPartClass(self.game.OKBALL, tBackbitmap: ( self.OwnBitmap), bbx: 700, bby: 420);
          self.blockcenter = self.AddSubPart( tsubpart, 700, 420, 35, 35, 1);
        }
        tsubpart =  TextPartClass::new("Block Center", Font::new("Times New Roman", 13f, FontStyle.Regular, GraphicsUnit.Pixel), 150, 16, false);
        self.blockcentertext = self.AddSubPart( tsubpart, 750, 429, 150, 16, 0);
        if (self.dofirsttech == 0)
        {
          tsubpart =  new SteveButtonPartClass(self.game.CANCELBALL, tBackbitmap: ( self.OwnBitmap), bbx: 700, bby: 460);
          self.firsttech = self.AddSubPart( tsubpart, 700, 460, 35, 35, 1);
        }
        else
        {
          tsubpart =  new SteveButtonPartClass(self.game.OKBALL, tBackbitmap: ( self.OwnBitmap), bbx: 700, bby: 460);
          self.firsttech = self.AddSubPart( tsubpart, 700, 460, 35, 35, 1);
        }
        tsubpart =  TextPartClass::new("Full 1st Level Tech", Font::new("Times New Roman", 13f, FontStyle.Regular, GraphicsUnit.Pixel), 150, 16, false);
        self.firsttechtext = self.AddSubPart( tsubpart, 750, 469, 150, 16, 0);
        if (self.doallied == 0)
        {
          tsubpart =  new SteveButtonPartClass(self.game.CANCELBALL, tBackbitmap: ( self.OwnBitmap), bbx: 700, bby: 460);
          self.allied = self.AddSubPart( tsubpart, 700, 500, 35, 35, 1);
        }
        else
        {
          tsubpart =  new SteveButtonPartClass(self.game.OKBALL, tBackbitmap: ( self.OwnBitmap), bbx: 700, bby: 460);
          self.allied = self.AddSubPart( tsubpart, 700, 500, 35, 35, 1);
        }
        tsubpart =  TextPartClass::new("Allied AIs", Font::new("Times New Roman", 13f, FontStyle.Regular, GraphicsUnit.Pixel), 150, 16, false);
        self.alliedtext = self.AddSubPart( tsubpart, 750, 509, 150, 16, 0);
        bitmap: Bitmap = (Bitmap) null;
        tsubpart =  new TextButtonPartClass("M", 35, tBackbitmap: ( bitmap));
        self.masterfile = self.AddSubPart( tsubpart, 700, 540, 35, 35, 1);
        tsubpart =  TextPartClass::new("MASTER: " + self.domasterfile, Font::new("Times New Roman", 11f, FontStyle.Regular, GraphicsUnit.Pixel), 150, 16, false);
        self.masterfiletext = self.AddSubPart( tsubpart, 750, 549, 150, 16, 0);
        tsubpart =  new TextButtonPartClass("Make", 100, tBackbitmap: ( self.OwnBitmap), bbx: 50, bby: 485);
        self.BStartGameID = self.AddSubPart( tsubpart, 460, 700, 100, 35, 1);
        tsubpart =  new SteveButtonPartClass(self.game.BACKBUTTON, tBackbitmap: ( self.OwnBitmap), bbx: 30, bby: 700);
        self.cancelID = self.AddSubPart( tsubpart, 30, 700, 35, 35, 1);
      }
    }

    pub HandleMouseClick: WindowReturnClass(x: i32, y: i32, b: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      if (self.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 = self.SubPartCounter;
        for (let mut index: i32 = 0; index <= subPartCounter; index += 1)
        {
          if (x > self.SubPartX[index] & x < self.SubPartX[index] + self.SubPartW[index] && y > self.SubPartY[index] & y < self.SubPartY[index] + self.SubPartH[index])
          {
            let mut num1: i32 = self.SubPartID[index];
            if (num1 == self.cancelID)
            {
              windowReturnClass.AddCommand(1, 49);
              windowReturnClass.AddCommand(2, 55);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.tab1)
            {
              self.tabmode = 0;
              self.DoStuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.tab2)
            {
              self.tabmode = 1;
              self.DoStuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.BLadderID)
            {
              self.game.FormRef.Cursor = Cursors.WaitCursor;
              self.opt1v = 17;
              self.opt8v = 8;
              self.opt2v = 13;
              self.opt3v = 2;
              self.opt10v = 1;
              self.opt4v = 90;
              self.opt11v = 100;
              self.opt5v = 10;
              self.opt12v = 25;
              self.opt6v = 40;
              self.opt7v = 40;
              self.dooldkingdom = 0;
              self.domaploop = 0;
              self.doshrowd = 0;
              self.domirror = 1;
              self.doblockcenter = 1;
              self.doallied = 0;
              self.domasterfile = "OFFICIAL LADDER/Ladder.ptmaster";
              self.MakeRandomMap();
              let mut num2: i32 =  Interaction.MsgBox( "A random ladder map is succesfully made!", Title: ( "Shadow Empire : Planetary Conquest"));
              self.game.FormRef.Cursor = Cursors.Default;
              self.game.Data.EditPass = "Heinrici45";
              self.game.Data.Description = "";
              self.game.Data.Description += "This scenario is generated for competitive play. The winner is the player who has most points at the start of round 10. If both players have the same amount of points, a draw is declared.";
              self.game.Data.Description += "\r\n\r\nPlayers earn points by capturing victory polocations: i32 as well as destroying enemy troops. Each victory poheld: i32 is worth 100 points. Each enemy power podestroyed: i32 is worth 1 point.";
              self.game.Data.Description += "\r\n\r\nBoth players start with 5 producing towns, 1 HQ and a few border divisions.";
              self.game.Data.Description += "\r\n\r\nThe 2nd player has 50% extra production for the production arriving at the start of the second round to offset his disadvantage in being the second player to move.\r\n\r\nGood Luck!";
              self.game.Data.Name = "Random Ladder Scenario";
              self.game.Data.FOWOn = true;
              self.game.Data.PBEM = true;
              self.game.Data.PasswordsOn = true;
              self.game.Data.RegimeObj[0].ResPts = 0;
              self.game.Data.RegimeObj[1].ResPts = 0;
              self.FinalizeLadder();
              self.game.EditObj.ShownWelcome = true;
              windowReturnClass.AddCommand(3, 1);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.BStartGameID)
            {
              if (self.game.EditObj.ShortRandomScreen)
              {
                if (!(self.opt11v == 100 | self.opt11v == 300))
                  self.opt11v = 100;
                if (self.Swater == 0)
                {
                  self.opt4v = 100;
                  self.opt5v = 30;
                  self.opt12v = 5;
                  self.opt6v =  Math.Round(30.0 +  VBMath.Rnd() * 40.0);
                }
                if (self.Swater == 1)
                {
                  self.opt4v = 80;
                  self.opt5v = 45;
                  self.opt12v = 10;
                  self.opt6v =  Math.Round(40.0 +  VBMath.Rnd() * 30.0);
                }
                if (self.Swater == 2)
                {
                  self.opt4v = 66;
                  self.opt5v = 52;
                  self.opt12v = 12;
                  self.opt6v =  Math.Round(45.0 +  VBMath.Rnd() * 20.0);
                }
                if (self.Swater == 3)
                {
                  self.opt4v = 50;
                  self.opt5v = 60;
                  self.opt12v = 15;
                  self.opt7v =  Math.Round(50.0 +  VBMath.Rnd() * 10.0);
                }
                if (self.Swater == 4)
                {
                  self.opt4v = 35;
                  self.opt5v = 70;
                  self.opt12v = 18;
                  self.opt7v =  Math.Round(55.0 +  VBMath.Rnd() * 10.0);
                }
                if (self.Swater == 5)
                {
                  self.opt4v = 20;
                  self.opt5v = 80;
                  self.opt12v = 20;
                  self.opt7v =  Math.Round(60.0 +  VBMath.Rnd() * 10.0);
                }
                if (self.Sworldsize == 0)
                {
                  self.opt1v = 40;
                  self.opt2v = 20;
                  self.opt8v = 7;
                  self.opt5v =  Math.Round( self.opt5v / 8.0);
                }
                if (self.Sworldsize == 1)
                {
                  self.opt1v = 60;
                  self.opt2v = 30;
                  self.opt8v = 14;
                  self.opt5v =  Math.Round( self.opt5v / 5.0);
                }
                if (self.Sworldsize == 2)
                {
                  self.opt1v = 70;
                  self.opt2v = 40;
                  self.opt8v = 30;
                  self.opt5v =  Math.Round( self.opt5v / 2.0);
                }
                if (self.Sworldsize == 3)
                {
                  self.opt1v = 90;
                  self.opt2v = 60;
                  self.opt8v = 50;
                  self.opt5v = self.opt5v;
                }
                if (self.Sworldsize == 4)
                {
                  self.opt1v = 130;
                  self.opt2v = 90;
                  self.opt8v = 110;
                  self.opt5v *= 3;
                }
                if (self.Sworldsize == 5)
                {
                  self.opt1v = 170;
                  self.opt2v = 120;
                  self.opt8v = 150;
                  self.opt5v *= 5;
                }
                if (self.Swater == 0)
                  self.opt8v =  Math.Round( self.opt8v * 1.3);
                if (self.Swater == 1)
                  self.opt8v =  Math.Round( self.opt8v * 1.2);
                if (self.Swater == 2)
                  self.opt8v *= 1;
                if (self.Swater == 3)
                  self.opt8v =  Math.Round( self.opt8v * 0.8);
                if (self.Swater == 4)
                  self.opt8v =  Math.Round( self.opt8v * 0.66);
                if (self.Swater == 5)
                  self.opt8v =  Math.Round( self.opt8v * 0.5);
                if (self.Splayer == 0)
                  self.opt3v = 2;
                if (self.Splayer == 1)
                  self.opt3v = 3;
                if (self.Splayer == 2)
                  self.opt3v = 4;
                if (self.Splayer == 3)
                  self.opt3v = 5;
                if (self.Splayer == 4)
                  self.opt3v = 6;
                if (self.Splayer == 5)
                  self.opt3v = 7;
                if (self.Splayer == 6)
                  self.opt3v = 8;
                if (self.Splayer == 7)
                  self.opt3v = 9;
                if (self.Splayer == 8)
                  self.opt3v = 10;
                if (self.Splayer == 9)
                  self.opt3v = 11;
                if (self.Splayer == 10)
                  self.opt3v = 12;
                if (self.Splayer == 11)
                  self.opt3v = 13;
                if (self.Splayer == 12)
                  self.opt3v = 14;
                if (self.Spop == 1)
                {
                  self.opt8v =  Math.Round( self.opt8v / 2.0);
                  self.opt6v =  Math.Round( self.opt6v * 1.5);
                  self.opt7v =  Math.Round( self.opt7v * 1.33);
                }
                if (self.Spop == 2)
                {
                  self.opt8v =  Math.Round( self.opt8v / 4.0);
                  self.opt6v =  Math.Round( self.opt6v * 1.5);
                  self.opt7v =  Math.Round( self.opt7v * 1.33);
                }
                if (self.domaploop == 0)
                  self.doblockcenter = 1;
                self.opt6v =  Math.Round(20.0 +  VBMath.Rnd() * 20.0);
                self.opt7v =  Math.Round(20.0 +  VBMath.Rnd() * 20.0);
                self.opt10v = 2;
                self.opt12v = 30;
                if (self.optr2 == -2)
                {
                  self.opt5v = 0;
                  self.opt12v = 0;
                  self.opt6v = 0;
                }
                if (self.optr2 == -1)
                {
                  self.opt5v =  Math.Round( self.opt5v / 3.0);
                  self.opt12v =  Math.Round( self.opt12v / 5.0);
                  self.opt6v =  Math.Round( self.opt6v / 4.0);
                }
                if (self.optr2 == 1)
                {
                  self.opt5v =  Math.Round( self.opt5v * 1.5);
                  self.opt12v *= 2;
                  self.opt6v += 10;
                  self.opt6v =  Math.Round( self.opt6v * 1.5);
                }
                if (self.optr2 == 2)
                {
                  self.opt5v =  Math.Round( self.opt5v * 2.2);
                  self.opt12v *= 3;
                  self.opt6v += 20;
                  self.opt6v =  Math.Round( self.opt6v * 2.2);
                }
                if (self.optr3 == -2)
                {
                  self.opt7v += 20;
                  self.opt7v =  Math.Round( self.opt7v * 3.2);
                }
                if (self.optr3 == -1)
                {
                  self.opt7v += 10;
                  self.opt7v =  Math.Round( self.opt7v * 1.8);
                }
                if (self.optr3 == 1)
                  self.opt7v =  Math.Round( self.opt7v / 1.8);
                if (self.optr3 == 2)
                  self.opt7v =  Math.Round( self.opt7v / 3.2);
                self.domasterfile = self.game.EditObj.RandomUseMaster;
                self.dooldkingdom = 0;
                self.dooptimize = 0;
              }
              self.game.FormRef.Cursor = Cursors.WaitCursor;
              self.MakeRandomMap();
              if ( self.game.Data.RuleVar[418] > 0.0)
              {
                self.game.FormRef.Cursor = Cursors.Default;
                self.game.EditObj.ShownWelcome = true;
                windowReturnClass.AddCommand(3, 1);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              self.game.FormRef.Cursor = Cursors.Default;
            }
            else
            {
              if (num1 == self.opt1)
              {
                self.opt1v = self.SubPartList[index].Click(x - self.SubPartX[index], y - self.SubPartY[index], b);
                self.game.EditObj.ran1 = self.opt1v;
                self.game.EditObj.ranmem = 1;
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.opt2)
              {
                self.opt2v = self.SubPartList[index].Click(x - self.SubPartX[index], y - self.SubPartY[index], b);
                self.game.EditObj.ran2 = self.opt2v;
                self.game.EditObj.ranmem = 1;
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.opt3)
              {
                self.opt3v = self.SubPartList[index].Click(x - self.SubPartX[index], y - self.SubPartY[index], b);
                if (self.opt3v != 2)
                  self.domirror = 0;
                self.game.EditObj.ran3 = self.opt3v;
                self.game.EditObj.ranmem = 1;
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.opt4)
              {
                self.opt4v = self.SubPartList[index].Click(x - self.SubPartX[index], y - self.SubPartY[index], b);
                self.game.EditObj.ran4 = self.opt4v;
                self.game.EditObj.ranmem = 1;
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.opt5)
              {
                self.opt5v = self.SubPartList[index].Click(x - self.SubPartX[index], y - self.SubPartY[index], b);
                self.game.EditObj.ran5 = self.opt5v;
                self.game.EditObj.ranmem = 1;
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.opt6)
              {
                self.opt6v = self.SubPartList[index].Click(x - self.SubPartX[index], y - self.SubPartY[index], b);
                self.game.EditObj.ran6 = self.opt6v;
                self.game.EditObj.ranmem = 1;
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.opt7)
              {
                self.opt7v = self.SubPartList[index].Click(x - self.SubPartX[index], y - self.SubPartY[index], b);
                self.game.EditObj.ran7 = self.opt7v;
                self.game.EditObj.ranmem = 1;
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.opt8)
              {
                self.opt8v = self.SubPartList[index].Click(x - self.SubPartX[index], y - self.SubPartY[index], b);
                self.game.EditObj.ran8 = self.opt8v;
                self.game.EditObj.ranmem = 1;
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.opt9)
              {
                self.opt9v = self.SubPartList[index].Click(x - self.SubPartX[index], y - self.SubPartY[index], b);
                self.game.EditObj.ran9 = self.opt9v;
                self.game.EditObj.ranmem = 1;
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.opt10)
              {
                self.opt10v = self.SubPartList[index].Click(x - self.SubPartX[index], y - self.SubPartY[index], b);
                self.game.EditObj.ran10 = self.opt10v;
                self.game.EditObj.ranmem = 1;
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.opt11)
              {
                self.opt11v = self.SubPartList[index].Click(x - self.SubPartX[index], y - self.SubPartY[index], b);
                self.game.EditObj.ran11 = self.opt11v;
                self.game.EditObj.ranmem = 1;
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.opt12)
              {
                self.opt12v = self.SubPartList[index].Click(x - self.SubPartX[index], y - self.SubPartY[index], b);
                self.game.EditObj.ran12 = self.opt12v;
                self.game.EditObj.ranmem = 1;
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.optimize)
              {
                if (self.dooptimize == 0)
                {
                  self.dooptimize = 1;
                  self.game.EditObj.ranoptimize = 1;
                  self.game.EditObj.ranmem = 1;
                }
                else
                {
                  self.dooptimize = 0;
                  self.game.EditObj.ranoptimize = 0;
                  self.game.EditObj.ranmem = 1;
                }
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.shrowd)
              {
                if (self.doshrowd == 0)
                {
                  self.doshrowd = 1;
                  self.game.EditObj.randoshrowd = 1;
                  self.game.EditObj.ranmem = 1;
                }
                else
                {
                  self.doshrowd = 0;
                  self.game.EditObj.randoshrowd = 0;
                  self.game.EditObj.ranmem = 1;
                }
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.allied)
              {
                if (self.doallied == 0)
                {
                  self.doallied = 1;
                  self.game.EditObj.randoallied = 1;
                  self.game.EditObj.ranmem = 1;
                }
                else
                {
                  self.doallied = 0;
                  self.game.EditObj.randoallied = 0;
                  self.game.EditObj.ranmem = 1;
                }
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.mirror)
              {
                if (self.domirror == 0)
                {
                  self.domirror = 1;
                  self.game.EditObj.randomirror = 1;
                  self.game.EditObj.ranmem = 1;
                }
                else
                {
                  self.domirror = 0;
                  self.game.EditObj.randomirror = 0;
                  self.game.EditObj.ranmem = 1;
                }
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.firsttech)
              {
                if (self.dofirsttech == 0)
                {
                  self.dofirsttech = 1;
                  self.game.EditObj.randofirsttech = 1;
                  self.game.EditObj.ranmem = 1;
                }
                else
                {
                  self.dofirsttech = 0;
                  self.game.EditObj.randofirsttech = 0;
                  self.game.EditObj.ranmem = 1;
                }
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.z1)
              {
                self.Srawuse = self.Srawuse != 0 ? 0 : 1;
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.r1)
              {
                self.optr1 = -2;
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.r2)
              {
                self.optr1 = -1;
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.r3)
              {
                self.optr1 = 0;
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.r4)
              {
                self.optr1 = 1;
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.r5)
              {
                self.optr1 = 2;
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.r6)
              {
                self.optr2 = -2;
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.r7)
              {
                self.optr2 = -1;
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.r8)
              {
                self.optr2 = 0;
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.r9)
              {
                self.optr2 = 1;
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.r10)
              {
                self.optr2 = 2;
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.r11)
              {
                self.optr3 = -2;
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.r12)
              {
                self.optr3 = -1;
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.r13)
              {
                self.optr3 = 0;
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.r14)
              {
                self.optr3 = 1;
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.r15)
              {
                self.optr3 = 2;
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.r26)
              {
                self.optr6 = self.optr6 != 0 ? 0 : 1;
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.r27)
              {
                self.optr7 = self.optr7 != 0 ? 0 : 1;
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.r28)
              {
                self.optr8 = self.optr8 != 0 ? 0 : 1;
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.o1)
              {
                self.Sworldsize = 0;
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.o2)
              {
                self.Sworldsize = 1;
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.o3)
              {
                self.Sworldsize = 2;
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.o4)
              {
                self.Sworldsize = 3;
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.o5)
              {
                self.Sworldsize = 4;
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.o6)
              {
                self.Sworldsize = 5;
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.r16)
              {
                self.optr4 = self.optr4 != 0 ? 0 : 1;
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.r17)
              {
                self.optr5 = self.optr5 != 0 ? 0 : 1;
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.o7)
              {
                self.domaploop = self.domaploop != 0 ? 0 : 1;
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.o8)
              {
                self.domirror = self.domirror != 0 ? 0 : 1;
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.o19)
              {
                self.opt11v = self.opt11v != 300 ? 300 : 100;
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.o9)
              {
                self.doshrowd = self.doshrowd != 0 ? 0 : 1;
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.o10)
              {
                self.dosinglestart = self.dosinglestart != 0 ? 0 : 1;
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.o12)
              {
                self.dofirsttech = self.dofirsttech != 0 ? 0 : 1;
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.o18)
              {
                self.Spop = !(self.Spop == 0 | self.Spop == 1) ? 0 : 2;
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.o16)
              {
                self.Spop = !(self.Spop == 0 | self.Spop == 2) ? 0 : 1;
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.o17)
              {
                self.Sraw = self.Sraw != 0 ? 0 : 1;
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.o13)
              {
                if (self.Scrate == 0)
                {
                  self.Scrate = 1;
                }
                else
                {
                  self.Scrate = 0;
                  self.dosinglestart = 1;
                }
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.o14)
              {
                self.dostats = self.dostats != 0 ? 0 : 1;
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.o15)
              {
                self.doallied = self.doallied != 0 ? 0 : 1;
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.o11)
              {
                self.opt9v = self.opt9v != 0 ? 0 : 4;
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.w1)
              {
                self.Splayer = 0;
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.w2)
              {
                self.Splayer = 1;
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.w3)
              {
                self.Splayer = 2;
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.w4)
              {
                self.Splayer = 3;
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.w5)
              {
                self.Splayer = 4;
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.w6)
              {
                self.Splayer = 5;
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.w11)
              {
                self.Splayer = 6;
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.w12)
              {
                self.Splayer = 7;
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.w13)
              {
                self.Splayer = 8;
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.w14)
              {
                self.Splayer = 9;
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.w15)
              {
                self.Splayer = 10;
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.w16)
              {
                self.Splayer = 11;
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.w17)
              {
                self.Splayer = 12;
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.w7)
              {
                self.Sclimate = 0;
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.w8)
              {
                self.Sclimate = 1;
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.w9)
              {
                self.Sclimate = 2;
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.w10)
              {
                self.Sclimate = 3;
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.w21)
              {
                self.Sclimate = 4;
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.h1)
              {
                self.Swater = 0;
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.h2)
              {
                self.Swater = 1;
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.h3)
              {
                self.Swater = 2;
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.h4)
              {
                self.Swater = 3;
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.h5)
              {
                self.Swater = 4;
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.h6)
              {
                self.Swater = 5;
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.maploop)
              {
                if (self.domaploop == 0)
                {
                  self.domaploop = 1;
                  self.game.EditObj.randomaploop = 1;
                  self.game.EditObj.ranmem = 1;
                }
                else
                {
                  self.domaploop = 0;
                  self.game.EditObj.randomaploop = 0;
                  self.game.EditObj.ranmem = 1;
                }
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.oldkingdom)
              {
                if (self.dooldkingdom == 0)
                {
                  self.dooldkingdom = 1;
                  self.game.EditObj.ranoldkingdom = 1;
                  self.game.EditObj.ranmem = 1;
                }
                else
                {
                  self.dooldkingdom = 0;
                  self.game.EditObj.ranoldkingdom = 0;
                  self.game.EditObj.ranmem = 1;
                }
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.blockcenter)
              {
                if (self.doblockcenter == 0)
                {
                  self.doblockcenter = 1;
                  self.game.EditObj.randoblockcenter = 1;
                  self.game.EditObj.ranmem = 1;
                }
                else
                {
                  self.doblockcenter = 0;
                  self.game.EditObj.randoblockcenter = 0;
                  self.game.EditObj.ranmem = 1;
                }
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.z2 || num1 == self.masterfile)
              {
                str: String = self.game.HandyFunctionsObj.LoadSomething("SE1 Scenario file (*.se1)|*.se1", "Pick a masterfile to use...", self.game.AppPath + self.game.ModScenarioDir, true);
                if (File.Exists(self.game.AppPath + self.game.ModScenarioDir + str))
                {
                  self.domasterfile = str;
                  self.game.EditObj.ranmasterfile = self.domasterfile;
                  self.game.EditObj.RandomUseMaster = self.domasterfile;
                  self.game.EditObj.ranmem = 1;
                  self.DoStuff();
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                let mut num3: i32 =  Interaction.MsgBox( "Aborted.", Title: ( "Shadow Empire : Planetary Conquest"));
              }
            }
            return windowReturnClass;
          }
        }
        windowReturnClass.SetFlag(false);
        return windowReturnClass;
      }
      windowReturnClass.SetFlag(false);
      return windowReturnClass;
    }

    pub fn MakeRandomMap()
    {
      self.game.EditObj.ranmem = 1;
      self.game.EditObj.ranr1 = self.optr1;
      self.game.EditObj.ranr2 = self.optr2;
      self.game.EditObj.ranr3 = self.optr3;
      self.game.EditObj.ranr4 = self.optr4;
      self.game.EditObj.ranr5 = self.optr5;
      self.game.EditObj.ranr6 = self.optr6;
      self.game.EditObj.ranr7 = self.optr7;
      self.game.EditObj.ranr8 = self.optr8;
      self.game.EditObj.ran1 = self.opt1v;
      self.game.EditObj.ran2 = self.opt2v;
      self.game.EditObj.ran3 = self.opt3v;
      self.game.EditObj.ran4 = self.opt4v;
      self.game.EditObj.ran5 = self.opt5v;
      self.game.EditObj.ran6 = self.opt6v;
      self.game.EditObj.ran7 = self.opt7v;
      self.game.EditObj.ran8 = self.opt8v;
      self.game.EditObj.ran9 = self.opt9v;
      self.game.EditObj.ran10 = self.opt10v;
      self.game.EditObj.ran11 = self.opt11v;
      self.game.EditObj.ran12 = self.opt12v;
      self.game.EditObj.randomaploop = self.domaploop;
      self.game.EditObj.randoshrowd = self.doshrowd;
      self.game.EditObj.randomirror = self.domirror;
      self.game.EditObj.randoblockcenter = self.doblockcenter;
      self.game.EditObj.randofirsttech = self.dofirsttech;
      self.game.EditObj.ranmasterfile = self.domasterfile;
      self.game.EditObj.randoallied = self.doallied;
      self.game.EditObj.ranoldkingdom = self.dooldkingdom;
      self.game.EditObj.ranoptimize = self.dooptimize;
      self.game.EditObj.ransinglestart = self.dosinglestart;
      self.game.EditObj.ransworldsize = self.Sworldsize;
      self.game.EditObj.ransplayer = self.Splayer;
      self.game.EditObj.ranswater = self.Swater;
      self.game.EditObj.ransclimate = self.Sclimate;
      self.game.EditObj.ranscrate = self.Scrate;
      self.game.EditObj.ranstats = self.dostats;
      self.game.EditObj.ranpop = self.Spop;
      self.game.EditObj.ranraw = self.Sraw;
      if (self.domirror == 1)
      {
        if (self.opt1v % 2 == 0)
          this += 1.opt1v;
        if (self.opt2v % 2 == 0)
          this += 1.opt2v;
        if (self.opt8v % 2 > 0)
          this += 1.opt8v;
      }
      let mut opt1v: i32 = self.opt1v;
      let mut opt2v: i32 = self.opt2v;
      let mut opt3v: i32 = self.opt3v;
      let mut opt4v: i32 = self.opt4v;
      self.landcur = 0;
      let mut num1: i32 =  Math.Round(Conversion.Int( opt4v / 5.0 * ( self.opt7v / 100.0)));
      self.mountaincur = 0;
      self.game.SelectX = 0;
      self.game.SelectY = 0;
      self.rivercur = 0;
      let mut num2: i32 = self.opt5v;
      if (self.domirror == 1)
        num2 =  Math.Round(Conversion.Int( num2 / 2.0));
      self.game.Data = DataClass::new();
      self.game.Data.MapObj = new MapClass[1];
      if ((opt1v + 10) % 2 == 0 & self.domaploop == 1)
        opt1v += 1;
      self.game.Data.MapObj[0] = new MapClass(0, 0, opt1v, opt2v);
      if (self.domaploop == 1)
        self.game.Data.MapObj[0].MapLoop = true;
      self.game.Data.MasterfileReadPeople = true;
      self.game.HandyFunctionsObj.LoadMasterFile(self.game.AppPath + self.game.ModScenarioDir + "/" + self.domasterfile, LoadDescription: self.game.EditObj.ShortRandomScreen);
      self.game.Data.FOWOn = true;
      if ( self.game.Data.RuleVar[418] < 1.0)
      {
        let mut num3: i32 =  Interaction.MsgBox( "The selected masterfile cannot be used to make a random game.", Title: ( "Shadow Empire : Planetary Conquest"));
      }
      else
      {
        if (opt1v + opt2v <= 100)
          ;
        BitmapStore.ReloadSystemGraphics(self.game.Data.SystemGfx);
        self.game.Data.LoadGraphics((Form1) null);
        if ( self.game.Data.RuleVar[560] > 0.0 & self.Scrate == 1)
          self.game.Data.GameSlot[ Math.Round( self.game.Data.RuleVar[560])] = 1;
        if (self.Scrate == 1)
          self.game.Data.RuleVar[496] = 0.0f;
        if (self.doshrowd == 1)
          self.game.Data.CreatedWithShrowd = true;
        else
          self.game.Data.CreatedWithShrowd = false;
        self.WATER =  Math.Round( self.game.Data.RuleVar[401]);
        self.GRASS =  Math.Round( self.game.Data.RuleVar[402]);
        self.HIGHMOUNTAIN =  Math.Round( self.game.Data.RuleVar[403]);
        self.LOWMOUNTAIN =  Math.Round( self.game.Data.RuleVar[404]);
        self.LIGHTFOREST =  Math.Round( self.game.Data.RuleVar[405]);
        self.HEAVYFOREST =  Math.Round( self.game.Data.RuleVar[406]);
        self.SMALLRIVER =  Math.Round( self.game.Data.RuleVar[407]);
        self.BIGRIVER =  Math.Round( self.game.Data.RuleVar[408]);
        self.SWAMP =  Math.Round( self.game.Data.RuleVar[417]);
        self.URBAN =  self.game.Data.RuleVar[444] <= 0.0 ?  Math.Round( self.game.Data.RuleVar[402]) :  Math.Round( self.game.Data.RuleVar[444]);
        self.LIGHTURBAN = !( self.game.Data.RuleVar[445] > 0.0 & self.optr7 == 0) ?  Math.Round( self.game.Data.RuleVar[402]) :  Math.Round( self.game.Data.RuleVar[445]);
        self.FARMLAND = !( self.game.Data.RuleVar[446] > 0.0 & self.optr6 == 0) ?  Math.Round( self.game.Data.RuleVar[402]) :  Math.Round( self.game.Data.RuleVar[446]);
        if (self.optr2 == -2)
          self.FARMLAND = self.GRASS;
        let mut num4: i32 = opt1v;
        for (let mut index1: i32 = 0; index1 <= num4; index1 += 1)
        {
          let mut num5: i32 = opt2v;
          for (let mut index2: i32 = 0; index2 <= num5; index2 += 1)
          {
            if (self.opt4v == 100 | self.WATER == -1)
              self.game.Data.MapObj[0].HexObj[index1, index2] = new HexClass(self.GRASS, 0, 0);
            else
              self.game.Data.MapObj[0].HexObj[index1, index2] = new HexClass(self.WATER, 0, 0);
          }
        }
        if (self.optr8 > 0)
        {
          let mut itemTypeCounter: i32 = self.game.Data.ItemTypeCounter;
          for (let mut index3: i32 = 0; index3 <= itemTypeCounter; index3 += 1)
          {
            ItemTypeClass[] itemTypeObj = self.game.Data.ItemTypeObj;
            ItemTypeClass[] itemTypeClassArray = itemTypeObj;
            let mut index4: i32 = index3;
            let mut index5: i32 = index4;
            itemTypeClassArray[index5].ProdWeight = itemTypeObj[index4].ProdWeight * 3;
          }
        }
        if ( self.game.Data.RuleVar[481] > 0.0)
          self.MakeClimates();
        self.game.Data.AddRegime();
        self.game.Data.RemoveRegime(0);
        let mut num6: i32 = DrawMod.RandyNumber.Next(0, 9);
        let mut index6: i32 = opt3v - 1;
        let mut num7: i32 = index6;
        if (num7 < 6)
          num7 = 6;
        if (num7 > 6 & num7 < 13)
          num7 = 13;
        let mut num8: i32 = num7;
        for (let mut index7: i32 = 0; index7 <= num8; index7 += 1)
          self.Regid[index7] = index7;
        VBMath.Randomize();
        let mut num9: i32 = 0;
        do
        {
          let mut num10: i32 = num7;
          for (let mut index8: i32 = 0; index8 <= num10; index8 += 1)
          {
            let mut index9: i32 =  Math.Round( Conversion.Int(VBMath.Rnd() *  (num7 + 1)));
            let mut num11: i32 = self.Regid[index8];
            self.Regid[index8] = self.Regid[index9];
            self.Regid[index9] = num11;
          }
          num9 += 1;
        }
        while (num9 <= 10);
        if ( self.game.Data.RuleVar[496] > 0.0)
        {
          index6 += 1;
          let mut num12: i32 = num7 + 1;
          self.Regid[index6] = index6;
        }
        let mut num13: i32 = index6;
        for (let mut regnr: i32 = 0; regnr <= num13; regnr += 1)
        {
          if (regnr > 0 | self.game.Data.RegimeCounter < regnr)
            self.game.Data.AddRegime();
          self.game.Data.RegimeObj[regnr].Name = self.GetRandomRegimeName(regnr);
          self.game.Data.RegimeObj[regnr].People = 0;
          if ( self.game.Data.RuleVar[496] > 0.0 & index6 == regnr)
          {
            self.game.Data.RegimeObj[regnr].People =  Math.Round( self.game.Data.RuleVar[497]);
            self.game.Data.RegimeObj[regnr].DipBlock = true;
            self.game.Data.RegimeObj[regnr].Sleep = true;
          }
          if ( self.game.Data.RuleVar[457] == 0.0)
            self.game.Data.RegimeObj[regnr].ResPts = 20;
          else
            self.game.Data.RegimeObj[regnr].ResPts =  Math.Round( self.game.Data.RuleVar[457]);
          self.game.Data.RegimeObj[regnr].UnitName = "Division";
          self.game.Data.RegimeObj[regnr].HQName = "HQ";
          let mut num14: i32 = 1;
          let mut d: i32 = 0;
          while (num14 == 1)
          {
            num14 = 0;
            d += 1;
            self.game.Data.RegimeObj[regnr].Red = DrawMod.RandyNumber.Next(0, 235) + 20;
            self.game.Data.RegimeObj[regnr].Blue = DrawMod.RandyNumber.Next(0, 235) + 20;
            self.game.Data.RegimeObj[regnr].Green = DrawMod.RandyNumber.Next(0, 235) + 20;
            if ( (Math.Abs(self.game.Data.RegimeObj[regnr].Red - 0) + Math.Abs(self.game.Data.RegimeObj[regnr].Green - 0) + Math.Abs(self.game.Data.RegimeObj[regnr].Blue - 155)) <= 400.0 / Math.Sqrt(Math.Sqrt( d)) & d < 999)
              num14 = 1;
            let mut num15: i32 = regnr;
            for (let mut index10: i32 = 0; index10 <= num15; index10 += 1)
            {
              if (index10 != regnr)
              {
                if ( (Math.Abs(self.game.Data.RegimeObj[regnr].Red - self.game.Data.RegimeObj[index10].Red) + Math.Abs(self.game.Data.RegimeObj[regnr].Green - self.game.Data.RegimeObj[index10].Green) + Math.Abs(self.game.Data.RegimeObj[regnr].Blue - self.game.Data.RegimeObj[index10].Blue)) <= 400.0 / Math.Sqrt(Math.Sqrt( d)) & d < 999)
                  num14 = 1;
                if (Math.Abs(self.game.Data.RegimeObj[regnr].Red + self.game.Data.RegimeObj[regnr].Green + self.game.Data.RegimeObj[regnr].Blue) < 210 & d < 999)
                  num14 = 1;
              }
            }
          }
          self.game.Data.RegimeObj[regnr].Red2 =  byte.MaxValue;
          self.game.Data.RegimeObj[regnr].Green2 =  byte.MaxValue;
          self.game.Data.RegimeObj[regnr].Blue2 =  byte.MaxValue;
          self.game.Data.RegimeObj[regnr].TempCounter = (Bitmap) null;
          self.game.Data.RegimeObj[regnr].TempCounterHigh = (Bitmap) null;
          num6 += 1;
          if (Strings.Len(self.Flag1) == 0 & Strings.Len(self.Flag1b) == 0)
          {
            if (num6 == 1 | num6 == 13)
            {
              self.game.Data.RegimeObj[regnr].ReplaceHQSprite("default/national/Fantasy01Flag.png");
              self.game.Data.RegimeObj[regnr].ReplaceNationalSprite("default/national/Fantasy01ID.png");
            }
            if (num6 == 2 | num6 == 14)
            {
              self.game.Data.RegimeObj[regnr].ReplaceHQSprite("default/national/Fantasy02Flag.png");
              self.game.Data.RegimeObj[regnr].ReplaceNationalSprite("default/national/Fantasy02ID.png");
            }
            if (num6 == 3 | num6 == 15)
            {
              self.game.Data.RegimeObj[regnr].ReplaceHQSprite("default/national/Fantasy03Flag.png");
              self.game.Data.RegimeObj[regnr].ReplaceNationalSprite("default/national/Fantasy03ID.png");
            }
            if (num6 == 4 | num6 == 16)
            {
              self.game.Data.RegimeObj[regnr].ReplaceHQSprite("default/national/Fantasy04Flag.png");
              self.game.Data.RegimeObj[regnr].ReplaceNationalSprite("default/national/Fantasy04ID.png");
            }
            if (num6 == 5 | num6 == 17)
            {
              self.game.Data.RegimeObj[regnr].ReplaceHQSprite("default/national/Fantasy05Flag.png");
              self.game.Data.RegimeObj[regnr].ReplaceNationalSprite("default/national/Fantasy05ID.png");
            }
            if (num6 == 6 | num6 == 18)
            {
              self.game.Data.RegimeObj[regnr].ReplaceHQSprite("default/national/Fantasy06Flag.png");
              self.game.Data.RegimeObj[regnr].ReplaceNationalSprite("default/national/Fantasy06ID.png");
            }
            if (num6 == 7 | num6 == 19)
            {
              self.game.Data.RegimeObj[regnr].ReplaceHQSprite("default/national/BalkanFlag.png");
              self.game.Data.RegimeObj[regnr].ReplaceNationalSprite("default/national/BalkanID.png");
            }
            if (num6 == 8 | num6 == 20)
            {
              self.game.Data.RegimeObj[regnr].ReplaceHQSprite("default/national/CaliphateFlag.png");
              self.game.Data.RegimeObj[regnr].ReplaceNationalSprite("default/national/CaliphateID.png");
            }
            if (num6 == 9 | num6 == 21)
            {
              self.game.Data.RegimeObj[regnr].ReplaceHQSprite("default/national/EuroFlag.png");
              self.game.Data.RegimeObj[regnr].ReplaceNationalSprite("default/national/EuroID.png");
            }
            if (num6 == 10 | num6 == 22)
            {
              self.game.Data.RegimeObj[regnr].ReplaceHQSprite("default/national/starsandbarsFlag.png");
              self.game.Data.RegimeObj[regnr].ReplaceNationalSprite("default/national/starsandbarsID.png");
            }
            if (num6 == 11)
            {
              self.game.Data.RegimeObj[regnr].ReplaceHQSprite("default/national/secessionFlag.png");
              self.game.Data.RegimeObj[regnr].ReplaceNationalSprite("default/national/secessionID.png");
            }
            if (num6 == 12)
            {
              self.game.Data.RegimeObj[regnr].ReplaceHQSprite("default/national/FloridaFlag.png");
              self.game.Data.RegimeObj[regnr].ReplaceNationalSprite("default/national/FloridaID.png");
            }
          }
          else
          {
            self.game.Data.RegimeObj[regnr].ReplaceHQSprite(self.Flag1);
            self.game.Data.RegimeObj[regnr].ReplaceNationalSprite(self.Flag1b);
          }
          if ( self.game.Data.RuleVar[423] != 0.0)
          {
            num6 = self.Regid[regnr];
            if (num6 > 6)
              num6 -= 7;
            if ( self.game.Data.RuleVar[496] > 0.0 & regnr == self.opt3v)
            {
              self.game.Data.RegimeObj[regnr].ExtraGraphicUse =  Math.Round( self.game.Data.RuleVar[495]);
              if ( self.game.Data.RuleVar[422] > 0.0)
                self.game.Data.RegimeObj[regnr].OfficerPool =  Math.Round( self.game.Data.RuleVar[494]);
            }
            else if ( self.game.Data.RuleVar[491] != 0.0 & num6 == 6)
            {
              self.game.Data.RegimeObj[regnr].ExtraGraphicUse =  Math.Round( self.game.Data.RuleVar[491]);
              if ( self.game.Data.RuleVar[422] > 0.0)
                self.game.Data.RegimeObj[regnr].OfficerPool =  Math.Round( self.game.Data.RuleVar[490]);
            }
            else if ( self.game.Data.RuleVar[487] != 0.0 & num6 == 5)
            {
              self.game.Data.RegimeObj[regnr].ExtraGraphicUse =  Math.Round( self.game.Data.RuleVar[487]);
              if ( self.game.Data.RuleVar[422] > 0.0)
                self.game.Data.RegimeObj[regnr].OfficerPool =  Math.Round( self.game.Data.RuleVar[486]);
            }
            else if ( self.game.Data.RuleVar[483] != 0.0 & num6 == 4)
            {
              self.game.Data.RegimeObj[regnr].ExtraGraphicUse =  Math.Round( self.game.Data.RuleVar[483]);
              if ( self.game.Data.RuleVar[422] > 0.0)
                self.game.Data.RegimeObj[regnr].OfficerPool =  Math.Round( self.game.Data.RuleVar[482]);
            }
            else if ( self.game.Data.RuleVar[467] != 0.0 & num6 == 3)
            {
              self.game.Data.RegimeObj[regnr].ExtraGraphicUse =  Math.Round( self.game.Data.RuleVar[467]);
              if ( self.game.Data.RuleVar[422] > 0.0)
                self.game.Data.RegimeObj[regnr].OfficerPool =  Math.Round( self.game.Data.RuleVar[466]);
            }
            else if ( self.game.Data.RuleVar[433] != 0.0 & num6 == 2)
            {
              self.game.Data.RegimeObj[regnr].ExtraGraphicUse =  Math.Round( self.game.Data.RuleVar[433]);
              if ( self.game.Data.RuleVar[422] > 0.0)
                self.game.Data.RegimeObj[regnr].OfficerPool =  Math.Round( self.game.Data.RuleVar[432]);
            }
            else if ( self.game.Data.RuleVar[428] != 0.0 & num6 == 1)
            {
              self.game.Data.RegimeObj[regnr].ExtraGraphicUse =  Math.Round( self.game.Data.RuleVar[428]);
              if ( self.game.Data.RuleVar[422] > 0.0)
                self.game.Data.RegimeObj[regnr].OfficerPool =  Math.Round( self.game.Data.RuleVar[427]);
            }
            else
            {
              self.game.Data.RegimeObj[regnr].ExtraGraphicUse =  Math.Round( self.game.Data.RuleVar[423]);
              if ( self.game.Data.RuleVar[422] > 0.0)
                self.game.Data.RegimeObj[regnr].OfficerPool =  Math.Round( self.game.Data.RuleVar[422]);
            }
          }
          else if (DrawMod.RandyNumber.Next(0, 100) > 50)
            self.game.Data.RegimeObj[regnr].ExtraGraphicUse = -1;
          else if (self.game.Data.SFTypeObj[0].ExtraCounter > 1)
            self.game.Data.RegimeObj[regnr].ExtraGraphicUse = 2;
          else
            self.game.Data.RegimeObj[regnr].ExtraGraphicUse = -1;
          self.game.Data.RegimeObj[regnr].HQSpriteOverrule = true;
        }
        if (self.dooldkingdom > 0)
        {
          self.game.Data.AddPeople();
          self.game.Data.PeopleObj[self.game.Data.PeopleCounter].PeopleGroup = 99;
          self.game.Data.PeopleObj[self.game.Data.PeopleCounter].Name = "People's Republic";
          self.game.Data.PeopleObj[0].ProdMod[99] = 0.25f;
          self.game.Data.PeopleObj[0].BattleForMod[99] = 1f;
          self.game.Data.PeopleObj[0].BaseMorale[99] = 50;
          self.game.Data.PeopleObj[0].BattleVSMod[99] = 1f;
          self.game.Data.AddRegime();
          let mut index11: i32 = opt3v;
          self.game.Data.RegimeObj[index11].Name = "People's Republic";
          self.game.Data.RegimeObj[index11].People = self.game.Data.PeopleCounter;
          self.game.Data.RegimeObj[index11].ResPts = self.opt10v * self.opt8v + 20;
          self.game.Data.RegimeObj[index11].UnitName = "Division";
          self.game.Data.RegimeObj[index11].HQName = "HQ";
          self.game.Data.RegimeObj[index11].Red =  byte.MaxValue;
          self.game.Data.RegimeObj[index11].Blue = 120;
          self.game.Data.RegimeObj[index11].Green = 120;
          self.game.Data.RegimeObj[index11].Red2 =  byte.MaxValue;
          self.game.Data.RegimeObj[index11].Green2 =  byte.MaxValue;
          self.game.Data.RegimeObj[index11].Blue2 =  byte.MaxValue;
          self.game.Data.RegimeObj[index11].TempCounter = (Bitmap) null;
          self.game.Data.RegimeObj[index11].TempCounterHigh = (Bitmap) null;
          self.game.Data.RegimeObj[index11].ReplaceHQSprite("default/national/Flag Soviet.png");
          self.game.Data.RegimeObj[index11].ReplaceNationalSprite("default/national/National ID Soviet.png");
          if (self.game.Data.SFTypeObj[0].ExtraCounter > 0)
            self.game.Data.RegimeObj[index11].ExtraGraphicUse = 1;
          else
            self.game.Data.RegimeObj[index11].ExtraGraphicUse = -1;
          self.game.Data.RegimeObj[index11].HQSpriteOverrule = true;
        }
        while ( self.landcur <=  (opt1v * opt2v) * ( opt4v / 100.0))
        {
          let mut num16: i32 = 0;
          let mut num17: i32 = opt1v;
          for (let mut x: i32 = 0; x <= num17; x += 1)
          {
            let mut num18: i32 = opt2v;
            for (let mut y: i32 = 0; y <= num18; y += 1)
            {
              if (num16 == 0)
              {
                VBMath.Randomize();
                x =  Math.Round( Conversion.Int(VBMath.Rnd() *  (opt1v + 1)));
                y =  Math.Round( Conversion.Int(VBMath.Rnd() *  (opt2v + 1)));
              }
              let mut sizy: i32 =  VBMath.Rnd() <= 0.98 ? ( VBMath.Rnd() <= 0.9 ? ( VBMath.Rnd() <= 0.6 ?  Math.Round( Conversion.Int(VBMath.Rnd() *  opt4v)) :  Math.Round(Conversion.Int( VBMath.Rnd() *  VBMath.Rnd() * ( opt1v / 2.0 +  opt2v / 2.0)))) :  Math.Round( Conversion.Int(VBMath.Rnd() * VBMath.Rnd() *  (opt1v + opt2v)))) :  Math.Round(Conversion.Int( VBMath.Rnd() *  VBMath.Rnd() * ( (opt1v + opt2v) * ( opt4v / 10.0))));
              if (self.optr1 == -2)
                sizy =  Math.Round( sizy / 5.0);
              if (self.optr1 == -1)
                sizy =  Math.Round( sizy / 2.5);
              if (self.optr1 == 1)
                sizy =  Math.Round( sizy * 2.5);
              if (self.optr1 == 2)
                sizy *= 5;
              if (sizy < 1)
                sizy = 1;
              self.MakeLandBlob(x, y, sizy);
              num16 = 2;
              if (num16 == 2)
                break;
            }
            if (num16 == 2)
              break;
          }
        }
        if (self.HIGHMOUNTAIN > -1 & self.LOWMOUNTAIN > -1 && self.opt7v > 0)
        {
          let mut num19: i32 = 0;
          while ( self.mountaincur <=  (opt1v * opt2v) * ( num1 / 100.0) & num19 < 1000)
          {
            let mut num20: i32 = 0;
            let mut num21: i32 = opt1v;
            for (let mut x: i32 = 0; x <= num21; x += 1)
            {
              let mut num22: i32 = opt2v;
              for (let mut y: i32 = 0; y <= num22; y += 1)
              {
                if (num20 == 0)
                {
                  VBMath.Randomize();
                  x =  Math.Round( Conversion.Int(VBMath.Rnd() *  (opt1v + 1)));
                  y =  Math.Round( Conversion.Int(VBMath.Rnd() *  (opt2v + 1)));
                  num20 = 1;
                }
                let mut landscapeType: i32 = self.game.Data.MapObj[0].HexObj[x, y].LandscapeType;
                if (landscapeType == self.GRASS | landscapeType == self.LIGHTFOREST | landscapeType == self.HEAVYFOREST | landscapeType == self.SWAMP)
                {
                  x2: i32;
                  y2: i32;
                  if ( VBMath.Rnd() > 0.5)
                  {
                    x2 =  Math.Round( x + Conversion.Int( VBMath.Rnd() * ( (opt1v + opt2v) / 25.0)));
                    y2 =  Math.Round( ( y + Conversion.Int(VBMath.Rnd() * 2f)));
                  }
                  else
                  {
                    y2 =  Math.Round( y + Math.Max(2.0, Conversion.Int( VBMath.Rnd() * ( (opt1v + opt2v) / 60.0))));
                    x2 =  Math.Round( ( x + Math.Max(1f, Conversion.Int(VBMath.Rnd() * 1f))));
                  }
                  if (self.optr3 == -1)
                  {
                    if ( VBMath.Rnd() < 0.5)
                      x -= 3;
                    if ( VBMath.Rnd() < 0.5)
                      y -= 3;
                    if ( VBMath.Rnd() < 0.5)
                      x2 += 3;
                    if ( VBMath.Rnd() < 0.5)
                      y2 += 3;
                  }
                  if (self.optr3 == -2)
                  {
                    if ( VBMath.Rnd() < 0.5)
                      x -= 7;
                    if ( VBMath.Rnd() < 0.5)
                      y -= 7;
                    if ( VBMath.Rnd() < 0.5)
                      x2 += 7;
                    if ( VBMath.Rnd() < 0.5)
                      y2 += 7;
                  }
                  if (x < 0)
                    x = 0;
                  if (y < 0)
                    y = 0;
                  if (x2 > self.game.Data.MapObj[0].MapWidth)
                    x2 = self.game.Data.MapObj[0].MapWidth;
                  if (y2 > self.game.Data.MapObj[0].MapHeight)
                    y2 = self.game.Data.MapObj[0].MapHeight;
                  self.MakeMountainRange(x, y, x2, y2);
                  num20 = 2;
                  num19 += 1;
                }
                if (num20 == 2)
                  break;
              }
              if (num20 == 2)
                break;
            }
          }
        }
        Coordinate coordinate;
        if (self.SMALLRIVER > -1 & self.BIGRIVER > -1 && self.opt5v > 0)
        {
          self.rivstep = new int[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1, 7];
          self.nextrivstep = new Coordinate[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1, 7];
          let mut mapWidth: i32 = self.game.Data.MapObj[0].MapWidth;
          for (let mut index12: i32 = 0; index12 <= mapWidth; index12 += 1)
          {
            let mut mapHeight: i32 = self.game.Data.MapObj[0].MapHeight;
            for (let mut index13: i32 = 0; index13 <= mapHeight; index13 += 1)
            {
              let mut index14: i32 = 0;
              do
              {
                self.nextrivstep[index12, index13, index14].x = -1;
                index14 += 1;
              }
              while (index14 <= 5);
            }
          }
          self.MakeHeightTable();
          let mut num23: i32 = 0;
          while ( self.rivercur <=  (num2 * (self.game.Data.MapObj[0].MapWidth * self.game.Data.MapObj[0].MapHeight)) / 1000.0 & num23 < 6 * self.game.Data.MapObj[0].MapWidth * self.game.Data.MapObj[0].MapHeight)
          {
            VBMath.Randomize();
            let mut index15: i32 =  Math.Round( Conversion.Int(VBMath.Rnd() *  (opt1v + 1)));
            let mut index16: i32 =  Math.Round( Conversion.Int(VBMath.Rnd() *  (opt2v + 1)));
            let mut z1: i32 =  Math.Round( Conversion.Int(VBMath.Rnd() * 6f));
            num23 += 1;
            if (self.domirror == 1 &  index15 >=  self.game.Data.MapObj[0].MapWidth / 2.0)
              index15 = -1;
            if (index15 > -1)
            {
              let mut tfacing: i32 = 1;
              do
              {
                coordinate = self.game.HandyFunctionsObj.HexNeighbour(index15, index16, 0, tfacing);
                if (coordinate.onmap && self.game.HandyFunctionsObj.HasHexRiver(coordinate.x, coordinate.y, 0))
                  index15 = -1;
                tfacing += 1;
              }
              while (tfacing <= 6);
              if (index15 > -1 & index15 > 2 & index16 > 2 & index15 < self.game.Data.MapObj[0].MapWidth - 2 & index16 < self.game.Data.MapObj[0].MapHeight - 2 && !self.game.HandyFunctionsObj.HasHexRiver(index15, index16, 0) && self.game.Data.MapObj[0].HexObj[index15, index16].LandscapeType == self.LOWMOUNTAIN | self.game.Data.MapObj[0].HexObj[index15, index16].LandscapeType == self.HIGHMOUNTAIN)
              {
                let mut num24: i32 = 0;
                let mut index17: i32 = 0;
                do
                {
                  if (self.game.Data.MapObj[0].HexObj[index15, index16].RiverType[index17] > -1)
                    num24 += 1;
                  index17 += 1;
                }
                while (index17 <= 5);
                if (num24 == 0)
                {
                  this += 1.rivercur;
                  num23 = 0;
                  if ( self.game.Data.RuleVar[450] == 0.0)
                  {
                    self.DrawARiver(index15, index16, z1);
                  }
                  else
                  {
                    self.DrawARiver2(index15, index16, z1);
                    float num25 = 0.8f;
                    while ( VBMath.Rnd() <  num25)
                    {
                      num25 /= 2f;
                      index15 =  VBMath.Rnd() >= 0.5 ?  Math.Round( ( (index15 + 2) + VBMath.Rnd() * 2f)) :  Math.Round( ( (index15 - 2) + VBMath.Rnd() * 2f));
                      index16 =  VBMath.Rnd() >= 0.5 ?  Math.Round( ( (index16 + 2) + VBMath.Rnd() * 2f)) :  Math.Round( ( (index16 - 2) + VBMath.Rnd() * 2f));
                      let mut z2: i32 =  Math.Round( Conversion.Int(VBMath.Rnd() * 6f));
                      if (index15 > 1 & index16 > 1 & index15 < self.game.Data.MapObj[0].MapWidth & index16 < self.game.Data.MapObj[0].MapHeight)
                        self.DrawARiver2(index15, index16, z2);
                      else
                        num25 = 0.0f;
                    }
                  }
                  if (self.optr2 == 1 &  VBMath.Rnd() > 0.33)
                    self.MakeLakes(opt1v, opt2v);
                  else if (self.optr2 == 2)
                    self.MakeLakes(opt1v, opt2v);
                }
              }
            }
          }
        }
        if (self.domirror == 1)
          self.MirrorTheMap();
        if ( self.opt8v >  self.landcur / 10.0)
          self.opt8v =  Math.Round(Conversion.Int( self.landcur / 10.0));
        self.PlaceTowns(opt1v, opt2v);
        self.EnsureMountainPasses();
        self.DoSwamps();
        self.PlaceRegimes(opt1v, opt2v, opt3v);
        self.RESOURCESLOT = -1;
        if (self.Srawuse == 1 &&  self.game.Data.RuleVar[452] > 0.0)
          self.PlaceResources();
        if (self.dooptimize > 0)
          self.OptimizeForAI();
        self.PlaceRegimes2();
        if (self.Srawuse == 1 &&  self.game.Data.RuleVar[452] > 0.0)
          self.EqualizeResources();
        if (self.Srawuse == 0)
        {
          self.game.Data.RuleVar[452] = 0.0f;
          self.game.Data.RuleVar[822] = -1f;
          self.game.Data.RuleVar[823] = 0.0f;
          self.game.Data.RuleVar[824] = 0.0f;
          let mut index18: i32 = 0;
          do
          {
            self.game.Data.RegimeSlotShow[index18] = false;
            index18 += 1;
          }
          while (index18 <= 499);
          let mut itemTypeCounter: i32 = self.game.Data.ItemTypeCounter;
          for (let mut index19: i32 = 0; index19 <= itemTypeCounter; index19 += 1)
          {
            let mut index20: i32 = 0;
            do
            {
              self.game.Data.ItemTypeObj[index19].RegimeSlotsCost[index20] = -1;
              self.game.Data.ItemTypeObj[index19].RegimeSlotsCostQty[index20] = 0;
              index20 += 1;
            }
            while (index20 <= 4);
          }
          let mut sfTypeCounter: i32 = self.game.Data.SFTypeCounter;
          for (let mut index21: i32 = 0; index21 <= sfTypeCounter; index21 += 1)
          {
            self.game.Data.SFTypeObj[index21].FuelRegimeVar = -1;
            self.game.Data.SFTypeObj[index21].FuelForMove = 0;
            self.game.Data.SFTypeObj[index21].FuelForAttack = 0;
            self.game.Data.SFTypeObj[index21].OutOfFuelAttack = 0.0f;
            self.game.Data.SFTypeObj[index21].OutOfFuelDefense = 0.0f;
            self.game.Data.SFTypeObj[index21].OutOfFuelMove = 0.0f;
          }
        }
        self.game.Data.VPWin =  Math.Round(Conversion.Int( self.totvp * 0.8));
        if (self.dooldkingdom > 0)
          self.game.Data.VPWin = self.totvp;
        if (self.totvp == opt3v)
          self.game.Data.VPWin = opt3v;
        self.game.Data.MasterFile = self.domasterfile;
        self.game.Data.ASOn = true;
        if (self.doshrowd == 1)
        {
          self.game.Data.FOWOn = true;
          self.game.Data.ShrowdOn = true;
        }
        let mut regimeCounter1: i32 = self.game.Data.RegimeCounter;
        index22: i32;
        for (index22 = 0; index22 <= regimeCounter1; index22 += 1)
        {
          if (self.game.Data.NoAIAdvice)
            self.game.Data.RegimeObj[index22].AI = false;
          else
            self.game.Data.RegimeObj[index22].AI = true;
          let mut regimeCounter2: i32 = self.game.Data.RegimeCounter;
          for (let mut index23: i32 = 0; index23 <= regimeCounter2; index23 += 1)
          {
            if (index22 == index23)
              self.game.Data.RegimeObj[index22].RegimeRel[index23] = 1;
            else if ( self.game.Data.RuleVar[461] == 1.0)
              self.game.Data.RegimeObj[index22].RegimeRel[index23] = 1;
            else
              self.game.Data.RegimeObj[index22].RegimeRel[index23] = 0;
          }
        }
        self.game.Data.RegimeObj[0].AI = false;
        if (self.doallied == 1)
          self.game.Data.DoAllied = true;
        else
          self.game.Data.DoAllied = false;
        if (self.dofirsttech == 1)
        {
          let mut regimeCounter3: i32 = self.game.Data.RegimeCounter;
          for (index22 = 0; index22 <= regimeCounter3; index22 += 1)
          {
            let mut researchCounter: i32 = self.game.Data.ResearchCounter;
            for (let mut index24: i32 = 0; index24 <= researchCounter; index24 += 1)
            {
              if (self.game.Data.ResearchObj[index24].TechLevel == 1)
                self.game.Data.RegimeObj[index22].ResField[index24] = true;
            }
          }
        }
        let mut mapWidth1: i32 = self.game.Data.MapObj[0].MapWidth;
        for (let mut index25: i32 = 0; index25 <= mapWidth1; index25 += 1)
        {
          let mut mapHeight: i32 = self.game.Data.MapObj[0].MapHeight;
          for (let mut index26: i32 = 0; index26 <= mapHeight; index26 += 1)
          {
            if (self.game.Data.LandscapeTypeObj[self.game.Data.MapObj[0].HexObj[index25, index26].LandscapeType].IsSea)
            {
              let mut index27: i32 = 0;
              do
              {
                if (self.game.Data.MapObj[0].HexObj[index25, index26].RiverType[index27] > -1)
                {
                  self.game.Data.MapObj[0].HexObj[index25, index26].RiverType[index27] = -1;
                  coordinate = self.game.HandyFunctionsObj.HexNeighbourSameMap(index25, index26, self.game.EditObj.MapSelected, index27 + 1);
                  if (coordinate.onmap)
                    self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].RiverType[self.game.HandyFunctionsObj.HexFacing(coordinate.x, coordinate.y, coordinate.map, index25, index26, 0) - 1] = -1;
                }
                index27 += 1;
              }
              while (index27 <= 5);
            }
          }
        }
        let mut num26: i32 = 10;
        let mut num27: i32 = 10;
        let mut mapWidth2: i32 = self.game.Data.MapObj[0].MapWidth;
        for (let mut x1: i32 = 0; x1 <= mapWidth2; x1 += 1)
        {
          let mut mapHeight: i32 = self.game.Data.MapObj[0].MapHeight;
          for (let mut y1: i32 = 0; y1 <= mapHeight; y1 += 1)
          {
            if (self.game.Data.MapObj[0].HexObj[x1, y1].Location > -1)
            {
              let mut num28: i32 = 0;
              float num29 = 0.0f;
              let mut num30: i32 = 0;
              float num31 = 0.0f;
              let mut num32: i32 = 0;
              float num33 = 0.0f;
              let mut num34: i32 = 0;
              float num35 = 0.0f;
              if ( self.game.Data.LocObj[self.game.Data.MapObj[0].HexObj[x1, y1].Location].Type ==  self.game.Data.RuleVar[413])
              {
                num28 = 0;
                num29 = 0.0f;
                num30 = 1;
                num31 = 0.1f;
                num32 = 1;
                num33 = 1f;
                num34 = 2;
                num35 = 0.2f;
              }
              if ( self.game.Data.LocObj[self.game.Data.MapObj[0].HexObj[x1, y1].Location].Type ==  self.game.Data.RuleVar[414])
              {
                num28 = 0;
                num29 = 0.0f;
                num30 = 1;
                num31 = 0.2f;
                num32 = 1;
                num33 = 1f;
                num34 = 3;
                num35 = 0.2f;
              }
              if ( self.game.Data.LocObj[self.game.Data.MapObj[0].HexObj[x1, y1].Location].Type ==  self.game.Data.RuleVar[415])
              {
                num28 = 0;
                num29 = 0.0f;
                num30 = 1;
                num31 = 0.3f;
                num32 = 1;
                num33 = 1f;
                num34 = 3;
                num35 = 0.5f;
              }
              if ( self.game.Data.LocObj[self.game.Data.MapObj[0].HexObj[x1, y1].Location].Type ==  self.game.Data.RuleVar[416])
              {
                num28 = 0;
                num29 = 0.0f;
                num30 = 1;
                num31 = 0.35f;
                num32 = 2;
                num33 = 1f;
                num34 = 5;
                num35 = 0.2f;
              }
              if ( self.game.Data.LocObj[self.game.Data.MapObj[0].HexObj[x1, y1].Location].Type ==  self.game.Data.RuleVar[410])
              {
                num28 = 0;
                num29 = 0.0f;
                num30 = 1;
                num31 = 0.4f;
                num32 = 2;
                num33 = 1f;
                num34 = 5;
                num35 = 0.2f;
              }
              let mut num36: i32 = x1 - num26;
              let mut num37: i32 = x1 + num26;
              for (let mut x2: i32 = num36; x2 <= num37; x2 += 1)
              {
                let mut num38: i32 = y1 - num27;
                let mut num39: i32 = y1 + num27;
                for (let mut y2: i32 = num38; y2 <= num39; y2 += 1)
                {
                  if (x2 >= 0 & y2 >= 0 & x2 <= self.game.Data.MapObj[0].MapWidth & y2 <= self.game.Data.MapObj[0].MapHeight)
                  {
                    if (self.domirror == 1)
                    {
                      let mut num40: i32 =  Math.Round(Conversion.Int( (self.game.Data.MapObj[0].MapWidth + 1) / 2.0));
                      let mut num41: i32 =  Math.Round(Conversion.Int( (self.game.Data.MapObj[0].MapHeight + 1) / 2.0));
                      let mut num42: i32 = y2 < num41 ? (y2 >= num41 ? num41 : self.game.Data.MapObj[0].MapHeight - y2) : self.game.Data.MapObj[0].MapHeight - y2;
                      let mut num43: i32 = x2 < num40 ? (x2 >= num40 ? x2 : self.game.Data.MapObj[0].MapWidth - x2) : self.game.Data.MapObj[0].MapWidth - x2;
                    }
                    let mut num44: i32 = self.game.HandyFunctionsObj.Distance(x1, y1, 0, x2, y2, 0);
                    let mut landscapeType: i32 = self.game.Data.MapObj[0].HexObj[x2, y2].LandscapeType;
                    if (landscapeType == self.GRASS | landscapeType == self.LIGHTFOREST | landscapeType == self.HEAVYFOREST | landscapeType == self.SWAMP | landscapeType == self.FARMLAND | landscapeType == self.LOWMOUNTAIN | landscapeType == self.HIGHMOUNTAIN)
                    {
                      if ( VBMath.Rnd() <=  num29 & num44 <= num28 |  num44 <= Conversion.Int( num28 / 2.0))
                      {
                        self.game.Data.MapObj[0].HexObj[x2, y2].LandscapeType =  Math.Round( self.game.Data.RuleVar[444]);
                        self.game.Data.MapObj[0].HexObj[x2, y2].SpriteNr =  Math.Round( self.game.Data.RuleVar[447]);
                        if ( self.game.Data.RuleVar[463] > 0.0)
                          self.game.Data.MapObj[0].HexObj[x2, y2].AreaCode[ Math.Round( self.game.Data.RuleVar[463])] = self.game.Data.MapObj[0].HexObj[x1, y1].AreaCode[ Math.Round( self.game.Data.RuleVar[463])];
                      }
                      else if ( VBMath.Rnd() <=  num31 & num44 <= num30 & !(landscapeType == self.HEAVYFOREST &  VBMath.Rnd() < 0.25 | landscapeType == self.HIGHMOUNTAIN | landscapeType == self.LOWMOUNTAIN &  VBMath.Rnd() < 0.5))
                      {
                        self.game.Data.MapObj[0].HexObj[x2, y2].LandscapeType = self.LIGHTURBAN;
                        self.game.Data.MapObj[0].HexObj[x2, y2].SpriteNr = 0;
                        if ( self.game.Data.RuleVar[463] > 0.0)
                          self.game.Data.MapObj[0].HexObj[x2, y2].AreaCode[ Math.Round( self.game.Data.RuleVar[463])] = self.game.Data.MapObj[0].HexObj[x1, y1].AreaCode[ Math.Round( self.game.Data.RuleVar[463])];
                      }
                      else if ( VBMath.Rnd() <=  num33 & num44 <= num32 & !(landscapeType == self.HEAVYFOREST &  VBMath.Rnd() < 0.4 | landscapeType == self.HIGHMOUNTAIN | landscapeType == self.LOWMOUNTAIN &  VBMath.Rnd() < 0.3))
                      {
                        self.game.Data.MapObj[0].HexObj[x2, y2].LandscapeType = self.FARMLAND;
                        self.game.Data.MapObj[0].HexObj[x2, y2].SpriteNr = 0;
                      }
                      else if ( VBMath.Rnd() <=  num35 & num44 <= num34 & !(landscapeType == self.HEAVYFOREST &  VBMath.Rnd() < 0.6 | landscapeType == self.HIGHMOUNTAIN | landscapeType == self.LOWMOUNTAIN &  VBMath.Rnd() < 0.1))
                      {
                        self.game.Data.MapObj[0].HexObj[x2, y2].LandscapeType = self.FARMLAND;
                        self.game.Data.MapObj[0].HexObj[x2, y2].SpriteNr = 0;
                      }
                      if (self.game.Data.MapObj[0].HexObj[x2, y2].LandscapeType == -1)
                      {
                        self.game.Data.MapObj[0].HexObj[x2, y2].LandscapeType = self.GRASS;
                        self.game.Data.MapObj[0].HexObj[x2, y2].SpriteNr = 0;
                      }
                    }
                  }
                }
              }
            }
          }
        }
        let mut mapWidth3: i32 = self.game.Data.MapObj[0].MapWidth;
        for (let mut cx: i32 = 0; cx <= mapWidth3; cx += 1)
        {
          let mut mapHeight: i32 = self.game.Data.MapObj[0].MapHeight;
          for (let mut cy: i32 = 0; cy <= mapHeight; cy += 1)
          {
            let mut landscapeType1: i32 = self.game.Data.MapObj[0].HexObj[cx, cy].LandscapeType;
            if (landscapeType1 == self.HEAVYFOREST &  self.game.Data.RuleVar[448] == 1.0)
            {
              let mut tfacing: i32 = 1;
              do
              {
                coordinate = self.game.HandyFunctionsObj.HexNeighbour(cx, cy, 0, tfacing);
                if (coordinate.onmap)
                {
                  let mut landscapeType2: i32 = self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].LandscapeType;
                  if (landscapeType2 == self.GRASS | landscapeType2 == self.SWAMP | landscapeType2 == self.LIGHTURBAN | landscapeType2 == self.FARMLAND)
                    self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].LandscapeType = self.LIGHTFOREST;
                }
                tfacing += 1;
              }
              while (tfacing <= 6);
            }
            if (landscapeType1 == self.HIGHMOUNTAIN &  self.game.Data.RuleVar[449] == 1.0)
            {
              let mut tfacing: i32 = 1;
              do
              {
                coordinate = self.game.HandyFunctionsObj.HexNeighbour(cx, cy, 0, tfacing);
                if (coordinate.onmap)
                {
                  let mut landscapeType3: i32 = self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].LandscapeType;
                  if (landscapeType3 == self.GRASS | landscapeType3 == self.SWAMP | landscapeType3 == self.LIGHTURBAN | landscapeType3 == self.LIGHTFOREST | landscapeType3 == self.HEAVYFOREST | landscapeType3 == self.FARMLAND)
                    self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].LandscapeType = self.LOWMOUNTAIN;
                }
                tfacing += 1;
              }
              while (tfacing <= 6);
            }
          }
        }
        self.game.HandyFunctionsObj.randomizeLT();
        if (self.optr4 == 1 & self.opt9v > 1)
          self.opt9v = 1;
        if (self.opt9v > 0)
        {
          let mut num45: i32 = opt1v;
          for (let mut x: i32 = 0; x <= num45; x += 1)
          {
            let mut num46: i32 = opt2v;
            for (let mut y: i32 = 0; y <= num46; y += 1)
            {
              if (self.game.Data.MapObj[0].HexObj[x, y].Location > -1 & self.game.Data.MapObj[0].HexObj[x, y].VP > 0)
                self.MakeRoads(x, y, self.opt9v, false);
            }
          }
          let mut num47: i32 = opt1v;
          for (let mut x: i32 = 0; x <= num47; x += 1)
          {
            let mut num48: i32 = opt2v;
            for (let mut y: i32 = 0; y <= num48; y += 1)
            {
              if (self.game.Data.MapObj[0].HexObj[x, y].Location > -1 & self.game.Data.MapObj[0].HexObj[x, y].VP > 0 &&  self.game.Data.RuleVar[461] == 1.0 & self.optr4 == 0)
                self.MakeRoads(x, y, self.opt9v, true);
            }
          }
          let mut num49: i32 = opt1v;
          for (let mut x: i32 = 0; x <= num49; x += 1)
          {
            let mut num50: i32 = opt2v;
            for (let mut y: i32 = 0; y <= num50; y += 1)
            {
              if (self.game.Data.MapObj[0].HexObj[x, y].Location > -1 &&  self.game.Data.RuleVar[475] > 0.0 & self.optr4 == 0)
                self.MakeRoads2(x, y, self.opt9v);
            }
          }
        }
        if ( self.game.Data.RuleVar[451] == 0.0 && self.optr5 == 0)
          self.HarbourAssurance();
        if (self.opt9v > 0)
        {
          let mut num51: i32 = opt1v;
          for (let mut x: i32 = 0; x <= num51; x += 1)
          {
            let mut num52: i32 = opt2v;
            for (let mut y: i32 = 0; y <= num52; y += 1)
            {
              if (self.RESOURCESLOT > -1 && self.game.Data.MapObj[0].HexObj[x, y].AreaCode[self.RESOURCESLOT] > 0)
                self.MakeRoads(x, y, self.opt9v, false);
              if ( self.game.Data.RuleVar[445] > 0.0 & self.optr4 == 0 &&  self.game.Data.MapObj[0].HexObj[x, y].LandscapeType ==  self.game.Data.RuleVar[445])
                self.MakeRoads(x, y, self.opt9v, false, true);
            }
          }
        }
        self.EnsureMountainPasses2();
        self.PlaceRegimes3();
        if ( self.game.Data.RuleVar[419] > 0.0 &  self.game.Data.RuleVar[419] < 6.0)
          self.game.HandyFunctionsObj.MakeAutoLabels( Math.Round( self.game.Data.RuleVar[419]));
        if ( self.game.Data.RuleVar[420] > -1.0 &  self.game.Data.RuleVar[421] > 0.0 && self.dooldkingdom > 0)
        {
          let mut mapWidth4: i32 = self.game.Data.MapObj[0].MapWidth;
          for (let mut x: i32 = 0; x <= mapWidth4; x += 1)
          {
            let mut mapHeight: i32 = self.game.Data.MapObj[0].MapHeight;
            for (let mut y: i32 = 0; y <= mapHeight; y += 1)
            {
              if (!self.game.Data.LandscapeTypeObj[self.game.Data.MapObj[0].HexObj[x, y].LandscapeType].IsSea && self.game.Data.MapObj[0].HexObj[x, y].Regime == -1)
              {
                self.game.Data.MapObj[0].HexObj[x, y].Regime = self.game.Data.RegimeCounter;
                if (self.game.Data.MapObj[0].HexObj[x, y].Location > -1)
                {
                  let mut location: i32 = self.game.Data.MapObj[0].HexObj[x, y].Location;
                  let mut unr: i32 = self.game.Data.AddUnit(x, y, 0);
                  self.game.Data.UnitObj[unr].Name = "Garrison";
                  self.game.Data.UnitObj[unr].Regime = index22 - 1;
                  self.game.Data.UnitObj[unr].Supply = 500;
                  self.game.Data.UnitObj[unr].SOSupReqPercent = 100;
                  self.game.Data.UnitObj[unr].IsHQ = false;
                  self.game.Data.LocObj[location].HQ = -1;
                  let mut index28: i32 = self.game.Data.AddSF(unr);
                  self.game.Data.SFObj[index28].Type =  Math.Round( self.game.Data.RuleVar[420]);
                  self.game.Data.SFObj[index28].Qty =  Math.Round( self.game.Data.RuleVar[421]);
                  self.game.Data.SFObj[index28].Rdn = 100;
                  self.game.Data.SFObj[index28].People = 0;
                  self.game.Data.SFObj[index28].Xp = 25;
                  self.game.Data.SFObj[index28].Mor = 50;
                }
              }
            }
          }
        }
        self.game.Data.SupplyMultiplier = 1f;
        self.game.Data.PPMultiplier = 1f;
        self.game.Data.ResMod =  Math.Round( self.game.Data.RuleVar[464]);
        if ( self.game.Data.RuleVar[464] == 0.0)
          self.game.Data.ResMod = 150000;
        self.game.Data.ResMod *= self.game.Data.RegimeCounter;
        self.game.Data.ResCostMod = self.game.Data.RuleVar[465];
        if ( self.game.Data.RuleVar[465] == 0.0)
          self.game.Data.ResCostMod = 1f;
        self.game.Data.ResCostMod *=  self.opt11v / 100f;
        description: String = self.game.Data.Description;
        if (self.game.EditObj.ShortRandomScreen)
        {
          self.game.Data.Designer = "Random Algorithm Gold";
          self.game.Data.Name = "Random Scenario Gold";
        }
        else
        {
          self.game.Data.Designer = "Classic Random Alg.";
          self.game.Data.Name = "Classic Random Scn.";
        }
        if ( self.game.Data.RuleVar[496] < 1.0)
          self.game.Data.Description = "A " + Strings.Trim(Conversion.Str( (self.game.Data.RegimeCounter + 1))) + " player scenario.\r\n";
        else
          self.game.Data.Description = "A " + Strings.Trim(Conversion.Str( self.game.Data.RegimeCounter)) + " player scenario + a hidden AI regime to control any potential 'revolutionary' forces.\r\n";
        if (self.dooldkingdom == 1)
        {
          data: DataClass = self.game.Data;
          data.Description = data.Description + "The People's Republic holds almost all initial territory, but it is weak and only produces 25% of what other regimes can produce. You need " + Strings.Trim(Conversion.Str( self.game.Data.VPWin)) + " Victory Points (100% of total) to win.\r\n";
        }
        else
        {
          data: DataClass = self.game.Data;
          data.Description = data.Description + "You need " + Strings.Trim(Conversion.Str( self.game.Data.VPWin)) + " Victory Points (80% of total) to win.\r\n";
        }
        if (self.game.Data.DoAllied)
          self.game.Data.Description += "\r\nAll AI regimes will ally when game starts.\r\n";
        data1: DataClass = self.game.Data;
        data1.Description = data1.Description + Strings.Trim(Conversion.Str( self.opt4v)) + "% of map is land.\r\n";
        data2: DataClass = self.game.Data;
        data2.Description = data2.Description + "There are about " + Strings.Trim(Conversion.Str( self.opt8v)) + " towns on the map, excluding start cities. The size level of the towns is " + Strings.Trim(Conversion.Str( self.opt10v)) + "\r\n";
        data3: DataClass = self.game.Data;
        data3.Description = data3.Description + "River level is " + Strings.Trim(Conversion.Str( self.opt5v)) + ". " + Strings.Trim(Conversion.Str( self.opt6v)) + "% of land should be forest and " + Strings.Trim(Conversion.Str( self.opt7v)) + "% of land should be mountain.\r\n";
        if (self.opt11v > 100)
        {
          data4: DataClass = self.game.Data;
          data4.Description = data4.Description + "Research is " + Strings.Trim(Conversion.Str( (self.opt11v - 100))) + "% more expensive than normally expected.\r\n";
        }
        else if (self.opt11v < 100)
        {
          data5: DataClass = self.game.Data;
          data5.Description = data5.Description + "Research is " + Strings.Trim(Conversion.Str( (100 - self.opt11v))) + "% cheaper than you would normally expect.\r\n";
        }
        self.game.Data.Description += "\r\n";
        if (self.domirror == 1)
        {
          self.game.Data.Description += "This is a mirror map.";
          self.game.Data.Description += "\r\n";
        }
        if (self.doblockcenter == 1)
        {
          self.game.Data.Description += "BlockCenter has been used for placing regimes.";
          self.game.Data.Description += "\r\n";
        }
        if (self.game.Data.CreatedWithShrowd)
          self.game.Data.Description += "This random game was created with a shroud of darkness. The creator cannot have seen the map in advance.";
        else
          self.game.Data.Description += "This game was not created with shroud of darkness. Keep in mind that the creator might have peeked how the map looks if you play with shroud on.";
        self.game.Data.Description += "\r\n";
        data6: DataClass = self.game.Data;
        data6.Description = data6.Description + "Created with the masterfile: " + self.domasterfile;
        if (Operators.CompareString(self.domasterfile, "advanced.ptmaster", false) == 0)
        {
          self.game.Data.Description += "\r\n";
          self.game.Data.Description += "\r\n";
          self.game.Data.Description += "SEASONS";
          self.game.Data.Description += "\r\n";
          self.game.Data.Description += "If this option is activated. There will be 8 rounds of clear wheater. Followed by 2 rounds of mud. Followed by 4 rounds of winter. Followed by 2 rounds of mud. Then the cycle repeats. Mud halves movement of mechanized land troops and stops movement of air forces (caused by rain). Giving a small break to players with no air supriority. Winter halves the offensive strength of all land troops, but not of air or navy forces.";
          self.game.Data.Description += "\r\n";
          self.game.Data.Description += "\r\n";
          self.game.Data.Description += "REBELLIONS";
          self.game.Data.Description += "\r\n";
          self.game.Data.Description += "If this option is activated there is a 2% chance per town per round that rebels will appear near a town in question. If they do the rebels arrive in small level I infantry forces. Between 20-50 individuals.";
          self.game.Data.Description += "\r\n";
          self.game.Data.Description += "\r\n";
          self.game.Data.Description += "FACTORIES";
          self.game.Data.Description += "\r\n";
          self.game.Data.Description += "If enabled you will be able to build factories on plain,light forest and heavy forest hexes. Factories must be build at least 3 hexes appart from eachother. AI will not use this option.";
          self.game.Data.Description += "\r\n";
          self.game.Data.Description += "\r\n";
          self.game.Data.Description += "HARDCORE LOGISTICS";
          self.game.Data.Description += "\r\n";
          self.game.Data.Description += "If enabled then trucks will be double the price they normally are. Trains will be available. They are equal to trucks but they can carry only 5 troops themselves (trucks=20), they offer the same landcap though. Naval movement speed will be doubled. And most importantly your production can only arrive in the hex where it is produced. This means you will have to build HQs in every producing town. And do a lot of manual transferring. Supply still flows for free from HQ to HQ. (AI is not affected by this rule)";
          self.game.Data.Description += "\r\n";
          self.game.Data.Description += "\r\n";
          self.game.Data.Description += "NUCLEAR OPTION";
          self.game.Data.Description += "\r\n";
          self.game.Data.Description += "Allows you to research and build nuclear bombers and tactical nukes. Nuclear Bomber will waste any location except nuclear bunkers and underground factories. Nuclear bombers do not directly attack troops. Bombers will destroy infrastructure (almost always) completly and leave fallout. Tactical nukes will target troops and do relativly limited infrastructural dammage. Fallout spreads out every round. The radiation level on a hex is equal to the troops you will lose. Radiation levels drop 1 every round. The only protection from radiation is having your troops in the same hex as a nuclear bunker or an underground factory. However those locationtypes are expensive to build. The AI will not use nukes. It is advisable to also play with factories on if you play with the nuclear variant. ";
          self.game.Data.Description += "\r\n";
          self.game.Data.Description += "\r\n";
          self.game.Data.Description += "IMMEDIATE NUCLEAR TECH";
          self.game.Data.Description += "\r\n";
          self.game.Data.Description += "Game starts with every player possesing the Nuclear I researchfield (to build simple nuclear bombers).";
          self.game.Data.Description += "\r\n";
          self.game.Data.Description += "\r\n";
          self.game.Data.Description += "DIPLOMATICS";
          self.game.Data.Description += "\r\n";
          self.game.Data.Description += "More multiplayer fun. Game starts with every player at war. But with this option turned on you can offer and make peace. Also there are cards to give PP grants to other players. All statistics which are normally hidden about casualties, kills and production are visible for everybody. Also you can share your recon with your allies if you feel like it.";
        }
        if ((uint) self.dostats > 0U)
          self.game.Data.RuleVar[313] = 1f;
        if ( self.game.Data.RuleVar[499] > 0.0)
          self.game.Data.GameSlot[ Math.Round( self.game.Data.RuleVar[499])] = 1;
        if ( self.game.Data.RuleVar[480] > 0.0)
          self.game.EventRelatedObj.DoCheckSpecificEvent( Math.Round( self.game.Data.RuleVar[480]));
        self.SmallIslands();
        let mut mapWidth5: i32 = self.game.Data.MapObj[0].MapWidth;
        for (let mut cx: i32 = 0; cx <= mapWidth5; cx += 1)
        {
          let mut mapHeight: i32 = self.game.Data.MapObj[0].MapHeight;
          for (let mut cy: i32 = 0; cy <= mapHeight; cy += 1)
          {
            let mut index29: i32 = 0;
            do
            {
              if (self.game.Data.MapObj[0].HexObj[cx, cy].RiverType[index29] > -1)
              {
                coordinate = self.game.HandyFunctionsObj.HexNeighbour(cx, cy, 0, index29 + 1);
                if (coordinate.onmap)
                {
                  let mut index30: i32 = index29 + 3;
                  if (index30 > 5)
                    index30 -= 6;
                  self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].RiverType[index30] = self.game.Data.MapObj[0].HexObj[cx, cy].RiverType[index29];
                }
              }
              index29 += 1;
            }
            while (index29 <= 5);
          }
        }
        let mut regimeCounter4: i32 = self.game.Data.RegimeCounter;
        for (let mut index31: i32 = 0; index31 <= regimeCounter4; index31 += 1)
        {
          self.game.Data.RegimeObj[index31].LoadSprites();
          self.game.Data.RegimeObj[index31].DoTempCounter();
        }
        let mut mapWidth6: i32 = self.game.Data.MapObj[0].MapWidth;
        for (let mut index32: i32 = 0; index32 <= mapWidth6; index32 += 1)
        {
          let mut mapHeight: i32 = self.game.Data.MapObj[0].MapHeight;
          for (let mut index33: i32 = 0; index33 <= mapHeight; index33 += 1)
          {
            if (self.game.Data.LandscapeTypeObj[self.game.Data.MapObj[0].HexObj[index32, index33].LandscapeType].IsSea)
              self.game.Data.MapObj[0].HexObj[index32, index33].Regime = -1;
          }
        }
        let mut regimeCounter5: i32 = self.game.Data.RegimeCounter;
        for (let mut index34: i32 = 0; index34 <= regimeCounter5; index34 += 1)
        {
          if (Strings.InStr(self.game.Data.RegimeObj[index34].Name, "<x>") > 0)
          {
            let mut num53: i32 = 0;
            let mut num54: i32 = 0;
            let mut num55: i32 = 0;
            newValue: String = "";
            let mut mapWidth7: i32 = self.game.Data.MapObj[0].MapWidth;
            for (let mut index35: i32 = 0; index35 <= mapWidth7; index35 += 1)
            {
              let mut mapHeight: i32 = self.game.Data.MapObj[0].MapHeight;
              for (let mut index36: i32 = 0; index36 <= mapHeight; index36 += 1)
              {
                if (self.game.Data.MapObj[0].HexObj[index35, index36].Regime == index34 && self.game.Data.MapObj[0].HexObj[index35, index36].VP > num53 & self.game.Data.MapObj[0].HexObj[index35, index36].Location > -1)
                {
                  num53 = self.game.Data.MapObj[0].HexObj[index35, index36].VP;
                  num54 = index35;
                  num55 = index36;
                  newValue = self.game.Data.LocObj[self.game.Data.MapObj[0].HexObj[index35, index36].Location].Name;
                }
              }
            }
            self.game.Data.RegimeObj[index34].Name = self.game.Data.RegimeObj[index34].Name.Replace("<x>", newValue);
          }
        }
      }
    }

    pub fn SmallIslands()
    {
      let mut mapWidth: i32 = self.game.Data.MapObj[0].MapWidth;
      for (let mut cx: i32 = 0; cx <= mapWidth; cx += 1)
      {
        let mut mapHeight: i32 = self.game.Data.MapObj[0].MapHeight;
        for (let mut cy: i32 = 0; cy <= mapHeight; cy += 1)
        {
          if (!self.game.Data.LandscapeTypeObj[self.game.Data.MapObj[0].HexObj[cx, cy].LandscapeType].IsSea)
          {
            let mut num1: i32 = 0;
            let mut tfacing1: i32 = 1;
            Coordinate coordinate;
            do
            {
              coordinate = self.game.HandyFunctionsObj.HexNeighbour(cx, cy, 0, tfacing1);
              if (coordinate.onmap && !self.game.Data.LandscapeTypeObj[self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].LandscapeType].IsSea)
                num1 += 1;
              tfacing1 += 1;
            }
            while (tfacing1 <= 6);
            if (num1 == 0)
            {
              let mut num2: i32 =  Math.Round(2.0 + Conversion.Int( VBMath.Rnd() * 1.99));
label_15:
              if (num2 > 0)
              {
                let mut tfacing2: i32 = 1;
                do
                {
                  coordinate = self.game.HandyFunctionsObj.HexNeighbour(cx, cy, 0, tfacing2);
                  if (coordinate.onmap & num2 > 0 && self.game.Data.LandscapeTypeObj[self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].LandscapeType].IsSea)
                  {
                    self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].LandscapeType = self.game.Data.MapObj[0].HexObj[cx, cy].LandscapeType;
                    self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].Regime = self.game.Data.MapObj[0].HexObj[cx, cy].Regime;
                    self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].SpriteNr = self.game.Data.MapObj[0].HexObj[cx, cy].SpriteNr;
                    let mut index: i32 = 0;
                    do
                    {
                      self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].AreaCode[index] = self.game.Data.MapObj[0].HexObj[cx, cy].AreaCode[index];
                      index += 1;
                    }
                    while (index <= 9);
                    --num2;
                  }
                  tfacing2 += 1;
                }
                while (tfacing2 <= 6);
                goto label_15;
              }
            }
          }
        }
      }
    }

    pub fn MakeClimates()
    {
      let mut index1: i32 =  Math.Round( self.game.Data.RuleVar[481]);
      let mut num1: i32 = 3;
      numArray: Vec<i32> = new int[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      if ( self.game.Data.RuleVar[498] > 0.0 & self.Sclimate == 0)
        self.game.Data.GameSlot[ Math.Round( self.game.Data.RuleVar[498])] = 1;
      num2: i32;
      num3: i32;
      if (self.Sclimate == 0)
      {
        num2 = 2;
        num1 = 2;
        num3 = 0;
      }
      if (self.Sclimate == 1)
      {
        num2 = 1;
        num1 = 3;
        num3 = 3;
      }
      if (self.Sclimate == 2)
      {
        num2 = 1;
        num1 = 2;
        num3 = 1;
      }
      if (self.Sclimate == 3)
      {
        num2 = 3;
        num1 = 4;
        num3 = 1;
      }
      if (self.Sclimate == 4)
      {
        num2 = 2;
        num1 = 2;
        num3 = 0;
      }
      let mut mapWidth1: i32 = self.game.Data.MapObj[0].MapWidth;
      for (let mut index2: i32 = 0; index2 <= mapWidth1; index2 += 1)
      {
        let mut mapHeight: i32 = self.game.Data.MapObj[0].MapHeight;
        for (let mut index3: i32 = 0; index3 <= mapHeight; index3 += 1)
        {
          let mut num4: i32 = num3 + 1;
          for (let mut index4: i32 = 1; index4 <= num4; index4 += 1)
          {
            if ( index3 <=  self.game.Data.MapObj[0].MapHeight * ( index4 /  num3 + 1.0) &&  index3 >=  self.game.Data.MapObj[0].MapHeight * ( (index4 - 1) /  (num3 + 1)))
              self.game.Data.MapObj[0].HexObj[index2, index3].AreaCode[index1] = num2 + index4 - 1;
          }
        }
      }
      let mut num5: i32 =  Math.Round(Conversion.Int( self.game.Data.MapObj[0].MapWidth / 25.0));
      if (num5 == 0)
        num5 = 1;
      if (!(self.Sclimate == 1 | self.Sclimate == 3))
        return;
      let mut num6: i32 = num5;
      for (let mut index5: i32 = 1; index5 <= num6; index5 += 1)
      {
        num7: i32;
        num8: i32;
        if (self.Sclimate == 1)
        {
          num7 =  Math.Round( self.game.Data.MapObj[0].MapHeight * 0.625);
          num8 =  Math.Round( self.game.Data.MapObj[0].MapHeight * 0.875);
        }
        else if (self.Sclimate == 3)
        {
          num7 =  Math.Round( self.game.Data.MapObj[0].MapHeight * (5.0 / 16.0));
          num8 =  Math.Round( self.game.Data.MapObj[0].MapHeight * (11.0 / 16.0));
        }
        if (self.optr2 == 0)
        {
          let mut num9: i32 =  Math.Round( Conversion.Int(VBMath.Rnd() *  (self.game.Data.MapObj[0].MapWidth - 12)));
          let mut num10: i32 =  Math.Round( ( num9 + Conversion.Int( (2.0 +  VBMath.Rnd() * 20.0))));
          let mut mapWidth2: i32 = self.game.Data.MapObj[0].MapWidth;
          for (let mut index6: i32 = 0; index6 <= mapWidth2; index6 += 1)
          {
            let mut mapHeight: i32 = self.game.Data.MapObj[0].MapHeight;
            for (let mut index7: i32 = 0; index7 <= mapHeight; index7 += 1)
            {
              if (index6 >= num9 & index6 <= num10 && index7 >= num7 & index7 <= num8)
              {
                self.game.Data.MapObj[0].HexObj[index6, index7].AreaCode[index1] = 99;
                numArray[index6, index7] = 1;
              }
            }
          }
        }
        else if (self.optr2 == -1)
        {
          let mut num11: i32 = 0;
          let mut mapWidth3: i32 = self.game.Data.MapObj[0].MapWidth;
          let mut mapWidth4: i32 = self.game.Data.MapObj[0].MapWidth;
          for (let mut index8: i32 = 0; index8 <= mapWidth4; index8 += 1)
          {
            let mut mapHeight: i32 = self.game.Data.MapObj[0].MapHeight;
            for (let mut index9: i32 = 0; index9 <= mapHeight; index9 += 1)
            {
              if (index8 >= num11 & index8 <= mapWidth3 && index9 >= num7 & index9 <= num8)
              {
                self.game.Data.MapObj[0].HexObj[index8, index9].AreaCode[index1] = 99;
                numArray[index8, index9] = 1;
              }
            }
          }
        }
        else if (self.optr2 == -2)
        {
          num7 =  Math.Round( (0 + num7) / 2.0);
          num8 =  Math.Round( (num8 + self.game.Data.MapObj[0].MapHeight) / 2.0);
          let mut num12: i32 = 0;
          let mut mapWidth5: i32 = self.game.Data.MapObj[0].MapWidth;
          let mut mapWidth6: i32 = self.game.Data.MapObj[0].MapWidth;
          for (let mut index10: i32 = 0; index10 <= mapWidth6; index10 += 1)
          {
            let mut mapHeight: i32 = self.game.Data.MapObj[0].MapHeight;
            for (let mut index11: i32 = 0; index11 <= mapHeight; index11 += 1)
            {
              if (index10 >= num12 & index10 <= mapWidth5 && index11 >= num7 & index11 <= num8)
              {
                self.game.Data.MapObj[0].HexObj[index10, index11].AreaCode[index1] = 99;
                numArray[index10, index11] = 1;
              }
            }
          }
        }
        let mut num13: i32 = 1;
        do
        {
          let mut mapWidth7: i32 = self.game.Data.MapObj[0].MapWidth;
          for (let mut cx: i32 = 0; cx <= mapWidth7; cx += 1)
          {
            let mut mapHeight: i32 = self.game.Data.MapObj[0].MapHeight;
            for (let mut cy: i32 = 0; cy <= mapHeight; cy += 1)
            {
              if (self.game.Data.MapObj[0].HexObj[cx, cy].AreaCode[index1] == 99 & numArray[cx, cy] == num13)
              {
                let mut tfacing: i32 = 1;
                do
                {
                  Coordinate coordinate = self.game.HandyFunctionsObj.HexNeighbour(cx, cy, 0, tfacing);
                  if (coordinate.onmap && self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].AreaCode[index1] != 99 & numArray[coordinate.x, coordinate.y] < num13)
                  {
                    numArray[coordinate.x, coordinate.y] = num13;
                    if ( VBMath.Rnd() > 0.5)
                    {
                      numArray[coordinate.x, coordinate.y] = num13 + 1;
                      self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].AreaCode[index1] = 99;
                    }
                  }
                  tfacing += 1;
                }
                while (tfacing <= 6);
              }
            }
          }
          num13 += 1;
        }
        while (num13 <= 4);
      }
    }

    pub fn PlaceResources()
    {
      SimpleList simpleList = SimpleList::new();
      numArray: Vec<i32> = new int[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      let mut stringListById: i32 = self.game.HandyFunctionsObj.GetStringListByID( Math.Round( self.game.Data.RuleVar[452]));
      if (stringListById == -1)
        return;
      self.game.AIObj.InitFindContinent();
      let mut length: i32 = self.game.Data.StringListObj[stringListById].Length;
      for (let mut index1: i32 = 0; index1 <= length; index1 += 1)
      {
        let mut integer1: i32 = Conversions.ToInteger(self.game.Data.StringListObj[stringListById].Data[index1, 0]);
        let mut integer2: i32 = Conversions.ToInteger(self.game.Data.StringListObj[stringListById].Data[index1, 1]);
        let mut integer3: i32 = Conversions.ToInteger(self.game.Data.StringListObj[stringListById].Data[index1, 2]);
        let mut integer4: i32 = Conversions.ToInteger(self.game.Data.StringListObj[stringListById].Data[index1, 3]);
        let mut integer5: i32 = Conversions.ToInteger(self.game.Data.StringListObj[stringListById].Data[index1, 4]);
        if (self.RESOURCESLOT == -1)
          self.RESOURCESLOT = integer5;
        let mut integer6: i32 = Conversions.ToInteger(self.game.Data.StringListObj[stringListById].Data[index1, 5]);
        let mut integer7: i32 = Conversions.ToInteger(self.game.Data.StringListObj[stringListById].Data[index1, 6]);
        let mut integer8: i32 = Conversions.ToInteger(self.game.Data.StringListObj[stringListById].Data[index1, 7]);
        let mut integer9: i32 = Conversions.ToInteger(self.game.Data.StringListObj[stringListById].Data[index1, 8]);
        str: String = self.game.Data.StringListObj[stringListById].Data[index1, 9];
        let mut num1: i32 = self.game.HandyFunctionsObj.CountLandHexesOnMap(0);
        let mut num2: i32 =  Math.Round(Conversion.Int( integer9 * ( num1 / 1000.0)));
        let mut regimeCounter: i32 = self.game.Data.RegimeCounter;
        index2: i32;
        Coordinate coordinate;
        for (let mut index3: i32 = 0; index3 <= regimeCounter; index3 += 1)
        {
          let mut num3: i32 = 0;
          let mut num4: i32 = integer8;
          if (self.Sraw == 1)
            num4 =  Math.Round( num4 / 2.0);
          if (integer8 % 2 > 0 &  VBMath.Rnd() > 0.5)
            num4 += 1;
          let mut mapWidth: i32 = self.game.Data.MapObj[0].MapWidth;
          index4: i32;
          for (index4 = 0; index4 <= mapWidth; index4 += 1)
          {
            let mut mapHeight: i32 = self.game.Data.MapObj[0].MapHeight;
            for (index2 = 0; index2 <= mapHeight; index2 += 1)
            {
              if (self.game.Data.MapObj[0].HexObj[index4, index2].Regime == index3)
              {
                num3 = 1;
                break;
              }
            }
            if (num3 == 1)
              break;
          }
          if (num3 == 1)
          {
            let mut x2: i32 = index4;
            let mut y2: i32 = index2;
            let mut num5: i32 = 0;
            while (num4 > 0 & num5 < 9999)
            {
              num5 += 1;
              let mut index5: i32 =  Math.Round( ( (x2 - integer7) + VBMath.Rnd() * 2f *  integer7));
              index2 =  Math.Round( ( (y2 - integer7) + VBMath.Rnd() * 2f *  integer7));
              let mut num6: i32 = 0;
              let mut tfacing: i32 = 1;
              do
              {
                coordinate = self.game.HandyFunctionsObj.HexNeighbour(index5, index2, 0, tfacing);
                if (coordinate.onmap && self.game.HandyFunctionsObj.IsHexHarbourOrSea(coordinate.x, coordinate.y, coordinate.map))
                  num6 += 1;
                tfacing += 1;
              }
              while (tfacing <= 6);
              if (num6 >= 1)
                index5 = -1;
              if (index5 >= 0 & index2 >= 0 & index5 <= self.game.Data.MapObj[0].MapWidth & index2 <= self.game.Data.MapObj[0].MapHeight && self.game.Data.MapObj[0].HexObj[index5, index2].LandscapeType == integer1 | integer1 == -1 && self.game.Data.MapObj[0].HexObj[index5, index2].SpriteNr == integer2 | integer2 == -1 && self.game.AIObj.HexContinent[index5, index2] == self.game.AIObj.HexContinent[x2, y2] && self.game.Data.MapObj[0].HexObj[index5, index2].Location == -1 && self.game.Data.MapObj[0].HexObj[index5, index2].SpecialType <= -1 && self.game.HandyFunctionsObj.Distance(index5, index2, 0, x2, y2, 0) <= integer7)
              {
                --num4;
                self.game.Data.MapObj[0].HexObj[index5, index2].SpecialType = integer3;
                self.game.Data.MapObj[0].HexObj[index5, index2].SpecialSprite = integer4;
                self.game.Data.MapObj[0].HexObj[index5, index2].AreaCode[integer5] = integer6;
                self.game.Data.MapObj[0].HexObj[index5, index2].Name = str;
              }
            }
            if (num4 > 0 & integer1 > -1 & integer2 > -1)
            {
              let mut num7: i32 = 0;
              while (num4 > 0 & num7 < 9999)
              {
                num7 += 1;
                let mut index6: i32 =  Math.Round( ( (x2 - integer7) + VBMath.Rnd() * 2f *  integer7));
                index2 =  Math.Round( ( (y2 - integer7) + VBMath.Rnd() * 2f *  integer7));
                let mut num8: i32 = 0;
                let mut tfacing: i32 = 1;
                do
                {
                  coordinate = self.game.HandyFunctionsObj.HexNeighbour(index6, index2, 0, tfacing);
                  if (coordinate.onmap && self.game.HandyFunctionsObj.IsHexHarbourOrSea(coordinate.x, coordinate.y, coordinate.map))
                    num8 += 1;
                  tfacing += 1;
                }
                while (tfacing <= 6);
                if (num8 >= 1)
                  index6 = -1;
                if (index6 >= 0 & index2 >= 0 & index6 <= self.game.Data.MapObj[0].MapWidth & index2 <= self.game.Data.MapObj[0].MapHeight && self.game.AIObj.HexContinent[index6, index2] == self.game.AIObj.HexContinent[x2, y2] && self.game.Data.MapObj[0].HexObj[index6, index2].Location == -1 && self.game.Data.MapObj[0].HexObj[index6, index2].SpecialType <= -1 && self.game.HandyFunctionsObj.Distance(index6, index2, 0, x2, y2, 0) <= integer7)
                {
                  --num4;
                  self.game.Data.MapObj[0].HexObj[index6, index2].SpecialType = integer3;
                  self.game.Data.MapObj[0].HexObj[index6, index2].SpecialSprite = integer4;
                  self.game.Data.MapObj[0].HexObj[index6, index2].LandscapeType = integer1;
                  self.game.Data.MapObj[0].HexObj[index6, index2].SpriteNr = integer2;
                  self.game.Data.MapObj[0].HexObj[index6, index2].AreaCode[integer5] = integer6;
                  self.game.Data.MapObj[0].HexObj[index6, index2].Name = str;
                }
              }
            }
          }
        }
        let mut num9: i32 = 0;
        let mut num10: i32 = -1;
        let mut num11: i32 = num2;
        if (self.Sraw == 1)
          num11 =  Math.Round( num11 / 2.0);
        if (num2 % 2 > 0 &  VBMath.Rnd() > 0.5)
          num11 += 1;
        if (num11 > 0)
        {
          while (num11 > 0 & num9 < 9999)
          {
            let mut index7: i32 = -1;
            let mut index8: i32 = -1;
            long num12 = 0;
            num9 += 1;
            let mut num13: i32 = 1;
            index9: i32;
            do
            {
              index9 =  Math.Round( Conversion.Int(VBMath.Rnd() *  (self.game.Data.MapObj[0].MapWidth + 1)));
              index2 =  Math.Round( Conversion.Int(VBMath.Rnd() *  (self.game.Data.MapObj[0].MapHeight + 1)));
              let mut num14: i32 = 0;
              let mut tfacing: i32 = 1;
              do
              {
                coordinate = self.game.HandyFunctionsObj.HexNeighbour(index9, index2, 0, tfacing);
                if (coordinate.onmap && self.game.HandyFunctionsObj.IsHexHarbourOrSea(coordinate.x, coordinate.y, coordinate.map))
                  num14 += 1;
                tfacing += 1;
              }
              while (tfacing <= 6);
              if (num14 >= 1)
                index9 = -1;
              if (index9 >= 0 & index2 >= 0 & index9 <= self.game.Data.MapObj[0].MapWidth & index2 <= self.game.Data.MapObj[0].MapHeight && self.game.Data.MapObj[0].HexObj[index9, index2].LandscapeType == integer1 | integer1 == -1 && self.game.Data.MapObj[0].HexObj[index9, index2].SpriteNr == integer2 | integer2 == -1 && self.game.Data.MapObj[0].HexObj[index9, index2].Location == -1 && self.game.Data.MapObj[0].HexObj[index9, index2].SpecialType <= -1)
              {
                let mut num15: i32 = 999;
                let mut mapWidth: i32 = self.game.Data.MapObj[0].MapWidth;
                for (let mut x2: i32 = 0; x2 <= mapWidth; x2 += 1)
                {
                  let mut mapHeight: i32 = self.game.Data.MapObj[0].MapHeight;
                  for (let mut y2: i32 = 0; y2 <= mapHeight; y2 += 1)
                  {
                    if (self.game.Data.MapObj[0].HexObj[x2, y2].AreaCode[integer5] == integer6)
                    {
                      let mut num16: i32 = self.game.HandyFunctionsObj.Distance(index9, index2, 0, x2, y2, 0, 20);
                      if (num16 < num15)
                        num15 = num16;
                    }
                  }
                }
                if ((long) num15 > num12 & num15 > 4)
                {
                  num12 = (long) num15;
                  index7 = index9;
                  index8 = index2;
                }
              }
              num13 += 1;
            }
            while (num13 <= 100);
            if (index7 > -1)
            {
              num17: i32;
              if (self.domirror == 1)
              {
                if (num10 == -1)
                {
                  let mut num18: i32 =  Math.Round(Conversion.Int( (self.game.Data.MapObj[0].MapWidth + 1) / 2.0));
                  let mut num19: i32 =  Math.Round(Conversion.Int( (self.game.Data.MapObj[0].MapHeight + 1) / 2.0));
                  let mut num20: i32 = index2 < num19 ? (index2 >= num19 ? num19 : self.game.Data.MapObj[0].MapHeight - index8) : self.game.Data.MapObj[0].MapHeight - index8;
                  num10 = index9 < num18 ? (index9 >= num18 ? num18 : self.game.Data.MapObj[0].MapWidth - index7) : self.game.Data.MapObj[0].MapWidth - index7;
                  num17 = num20;
                }
                else
                {
                  index7 = num10;
                  index8 = num17;
                  num10 = -1;
                  num17 = -1;
                }
              }
              --num11;
              self.game.Data.MapObj[0].HexObj[index7, index8].SpecialType = integer3;
              self.game.Data.MapObj[0].HexObj[index7, index8].SpecialSprite = integer4;
              self.game.Data.MapObj[0].HexObj[index7, index8].AreaCode[integer5] = integer6;
              self.game.Data.MapObj[0].HexObj[index7, index8].Name = str;
            }
          }
        }
        if (num11 > 0)
        {
          let mut num21: i32 = 0;
          while (num11 > 0 & integer1 > -1 & integer2 > -1 & num21 < 9999)
          {
            let mut num22: i32 = num21 + 1;
            let mut index10: i32 = -1;
            let mut index11: i32 = -1;
            long num23 = 0;
            num21 = num22 + 1;
            let mut num24: i32 = 1;
            do
            {
              let mut index12: i32 =  Math.Round( Conversion.Int(VBMath.Rnd() *  (self.game.Data.MapObj[0].MapWidth + 1)));
              index2 =  Math.Round( Conversion.Int(VBMath.Rnd() *  (self.game.Data.MapObj[0].MapHeight + 1)));
              let mut num25: i32 = 0;
              let mut tfacing: i32 = 1;
              do
              {
                coordinate = self.game.HandyFunctionsObj.HexNeighbour(index12, index2, 0, tfacing);
                if (coordinate.onmap && self.game.HandyFunctionsObj.IsHexHarbourOrSea(coordinate.x, coordinate.y, coordinate.map))
                  num25 += 1;
                tfacing += 1;
              }
              while (tfacing <= 6);
              if (num25 >= 1)
                index12 = -1;
              if (index12 >= 0 & index2 >= 0 & index12 <= self.game.Data.MapObj[0].MapWidth & index2 <= self.game.Data.MapObj[0].MapHeight && self.game.Data.MapObj[0].HexObj[index12, index2].Location == -1 && !self.game.Data.LandscapeTypeObj[self.game.Data.MapObj[0].HexObj[index12, index2].LandscapeType].IsSea && self.game.Data.MapObj[0].HexObj[index12, index2].SpecialType <= -1)
              {
                let mut num26: i32 = 999;
                let mut mapWidth: i32 = self.game.Data.MapObj[0].MapWidth;
                for (let mut x2: i32 = 0; x2 <= mapWidth; x2 += 1)
                {
                  let mut mapHeight: i32 = self.game.Data.MapObj[0].MapHeight;
                  for (let mut y2: i32 = 0; y2 <= mapHeight; y2 += 1)
                  {
                    if (self.game.Data.MapObj[0].HexObj[x2, y2].AreaCode[integer5] == integer6)
                    {
                      let mut num27: i32 = self.game.HandyFunctionsObj.Distance(index12, index2, 0, x2, y2, 0, 20);
                      if (num27 < num26)
                        num26 = num27;
                    }
                  }
                }
                if ((long) num26 > num23)
                {
                  num23 = (long) num26;
                  index10 = index12;
                  index11 = index2;
                }
              }
              num24 += 1;
            }
            while (num24 <= 1000);
            if (index10 > -1)
            {
              --num11;
              self.game.Data.MapObj[0].HexObj[index10, index11].SpecialType = integer3;
              self.game.Data.MapObj[0].HexObj[index10, index11].SpecialSprite = integer4;
              self.game.Data.MapObj[0].HexObj[index10, index11].LandscapeType = integer1;
              self.game.Data.MapObj[0].HexObj[index10, index11].SpriteNr = integer2;
              self.game.Data.MapObj[0].HexObj[index10, index11].AreaCode[integer5] = integer6;
              self.game.Data.MapObj[0].HexObj[index10, index11].Name = str;
            }
          }
        }
      }
    }

    pub fn EqualizeResources()
    {
      SimpleList simpleList1 = SimpleList::new();
      numArray1: Vec<i32> = new int[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      int[] numArray2 = new int[self.game.Data.RegimeCounter + 1];
      let mut num1: i32 =  Math.Round( self.game.Data.RuleVar[481]);
      num2: i32;
      while (num2 == 0 & num1 > 0)
      {
        num2 = 1;
        int[] numArray3 = new int[self.game.Data.RegimeCounter + 1];
        let mut mapWidth1: i32 = self.game.Data.MapObj[0].MapWidth;
        for (let mut index1: i32 = 0; index1 <= mapWidth1; index1 += 1)
        {
          let mut mapHeight: i32 = self.game.Data.MapObj[0].MapHeight;
          for (let mut index2: i32 = 0; index2 <= mapHeight; index2 += 1)
          {
            let mut index3: i32 = 0;
            do
            {
              if (self.game.Data.MapObj[0].HexObj[index1, index2].AreaCode[index3] > 0 & index3 != num1 && self.game.Data.MapObj[0].HexObj[index1, index2].Regime > -1)
              {
                int[] numArray4 = numArray3;
                int[] numArray5 = numArray4;
                let mut regime: i32 = self.game.Data.MapObj[0].HexObj[index1, index2].Regime;
                let mut index4: i32 = regime;
                let mut num3: i32 = numArray4[regime] + 1;
                numArray5[index4] = num3;
              }
              index3 += 1;
            }
            while (index3 <= 9);
          }
        }
        let mut num4: i32 = 999999;
        let mut num5: i32 = -1;
        let mut num6: i32 = -1;
        let mut num7: i32 = -1;
        let mut regimeCounter: i32 = self.game.Data.RegimeCounter;
        if (self.Scrate < 1)
          --regimeCounter;
        let mut num8: i32 = regimeCounter;
        for (let mut index: i32 = 0; index <= num8; index += 1)
        {
          if (numArray3[index] < num4 & numArray3[index] > 0)
          {
            num4 = numArray3[index];
            num6 = index;
          }
          if (numArray3[index] > num5)
          {
            num5 = numArray3[index];
            num7 = index;
          }
        }
        let mut tid: i32 = 0;
        if (num5 > num4)
        {
          SimpleList simpleList2 = SimpleList::new();
          let mut mapWidth2: i32 = self.game.Data.MapObj[0].MapWidth;
          for (let mut tdata1: i32 = 0; tdata1 <= mapWidth2; tdata1 += 1)
          {
            let mut mapHeight: i32 = self.game.Data.MapObj[0].MapHeight;
            for (let mut tdata2: i32 = 0; tdata2 <= mapHeight; tdata2 += 1)
            {
              let mut index: i32 = 0;
              do
              {
                if (self.game.Data.MapObj[0].HexObj[tdata1, tdata2].AreaCode[index] > 0 & index != num1 && self.game.Data.MapObj[0].HexObj[tdata1, tdata2].Regime == num7)
                {
                  tid += 1;
                  simpleList2.Add(tid, 1, tdata1, tdata2);
                }
                index += 1;
              }
              while (index <= 9);
            }
          }
          if (simpleList2.Counter > -1)
          {
            let mut index5: i32 =  Math.Round( (VBMath.Rnd() *  (simpleList2.Counter + 1)));
            let mut index6: i32 = simpleList2.Data1[index5];
            let mut index7: i32 = simpleList2.Data2[index5];
            self.game.Data.MapObj[0].HexObj[index6, index7].SpecialType = -1;
            self.game.Data.MapObj[0].HexObj[index6, index7].SpecialSprite = -1;
            let mut index8: i32 = 0;
            do
            {
              if (num1 != index8 | num1 == 0)
                self.game.Data.MapObj[0].HexObj[index6, index7].AreaCode[index8] = 0;
              index8 += 1;
            }
            while (index8 <= 9);
            self.game.Data.MapObj[0].HexObj[index6, index7].Name = "";
            num2 = 0;
          }
        }
      }
    }

    pub fn DoSwamps()
    {
      SimpleList simpleList = SimpleList::new();
      numArray: Vec<i32> = new int[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      if (self.opt12v <= 0)
        return;
      let mut mapWidth: i32 = self.game.Data.MapObj[0].MapWidth;
      for (let mut index1: i32 = 0; index1 <= mapWidth; index1 += 1)
      {
        let mut mapHeight: i32 = self.game.Data.MapObj[0].MapHeight;
        for (let mut index2: i32 = 0; index2 <= mapHeight; index2 += 1)
        {
          if (self.game.Data.MapObj[0].HexObj[index1, index2].LandscapeType == self.GRASS | self.game.Data.MapObj[0].HexObj[index1, index2].LandscapeType == self.LIGHTFOREST | self.game.Data.MapObj[0].HexObj[index1, index2].LandscapeType == self.HEAVYFOREST)
          {
            let mut num1: i32 = 0;
            let mut tfacing: i32 = 1;
            do
            {
              Coordinate coordinate = self.game.HandyFunctionsObj.HexNeighbourSameMap(index1, index2, self.game.EditObj.MapSelected, tfacing);
              if (coordinate.onmap)
              {
                if (self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].LandscapeType == self.WATER)
                  num1 += 6;
                if (self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].LandscapeType == self.SWAMP)
                  num1 += 3;
                if (self.game.Data.MapObj[0].HexObj[index1, index2].RiverType[tfacing - 1] == self.SMALLRIVER)
                  num1 += 10;
                if (self.game.Data.MapObj[0].HexObj[index1, index2].RiverType[tfacing - 1] == self.BIGRIVER)
                  num1 += 20;
                let mut num2: i32 = index1 - 3;
                let mut num3: i32 = index1 + 3;
                for (let mut x2: i32 = num2; x2 <= num3; x2 += 1)
                {
                  let mut num4: i32 = index2 - 3;
                  let mut num5: i32 = index2 + 3;
                  for (let mut y2: i32 = num4; y2 <= num5; y2 += 1)
                  {
                    if (x2 >= 0 & y2 >= 0 && x2 <= self.game.Data.MapObj[0].MapWidth & y2 <= self.game.Data.MapObj[0].MapHeight)
                    {
                      if (self.game.Data.MapObj[0].HexObj[x2, y2].LandscapeType == self.LOWMOUNTAIN)
                        num1 =  Math.Round( num1 - 5.0 / Math.Pow( self.game.HandyFunctionsObj.Distance(index1, index2, 0, x2, y2, 0), 2.0));
                      if (self.game.Data.MapObj[0].HexObj[x2, y2].LandscapeType == self.HIGHMOUNTAIN)
                        num1 =  Math.Round( num1 - 10.0 / Math.Pow( self.game.HandyFunctionsObj.Distance(index1, index2, 0, x2, y2, 0), 2.0));
                    }
                  }
                }
              }
              tfacing += 1;
            }
            while (tfacing <= 6);
            if (num1 > 0)
            {
              let mut num6: i32 =  Math.Round( num1 * ( self.opt12v / 10.0));
              if ( VBMath.Rnd() *  num6 >  VBMath.Rnd() * 100.0)
                self.game.Data.MapObj[0].HexObj[index1, index2].LandscapeType = self.SWAMP;
            }
          }
        }
      }
    }

    pub fn EnsureMountainPasses()
    {
      SimpleList simpleList = SimpleList::new();
      numArray: Vec<i32> = new int[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      let mut mapWidth: i32 = self.game.Data.MapObj[0].MapWidth;
      for (let mut index1: i32 = 0; index1 <= mapWidth; index1 += 1)
      {
        let mut mapHeight: i32 = self.game.Data.MapObj[0].MapHeight;
        for (let mut index2: i32 = 0; index2 <= mapHeight; index2 += 1)
        {
          if (self.game.Data.MapObj[0].HexObj[index1, index2].LandscapeType == self.HIGHMOUNTAIN)
          {
            if (self.game.HandyFunctionsObj.HasHexRoad(index1, index2, 0))
            {
              self.game.Data.MapObj[0].HexObj[index1, index2].LandscapeType = self.LOWMOUNTAIN;
            }
            else
            {
              let mut num1: i32 = 0;
              let mut tfacing1: i32 = 1;
              Coordinate coordinate;
              do
              {
                coordinate = self.game.HandyFunctionsObj.HexNeighbourSameMap(index1, index2, 0, tfacing1);
                if (coordinate.onmap && self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].LandscapeType == self.HIGHMOUNTAIN)
                  num1 += 1;
                tfacing1 += 1;
              }
              while (tfacing1 <= 6);
              if (num1 > 1)
              {
                let mut num2: i32 =  Math.Round( Conversion.Int( ( VBMath.Rnd() *  num1 + 1.0)));
                let mut num3: i32 = 0;
                let mut tfacing2: i32 = 1;
                do
                {
                  coordinate = self.game.HandyFunctionsObj.HexNeighbourSameMap(index1, index2, 0, tfacing2);
                  if (coordinate.onmap && self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].LandscapeType == self.HIGHMOUNTAIN)
                  {
                    num3 += 1;
                    if (num3 == num2)
                      self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].LandscapeType = self.LOWMOUNTAIN;
                  }
                  tfacing2 += 1;
                }
                while (tfacing2 <= 6);
              }
            }
          }
        }
      }
    }

    pub fn EnsureMountainPasses2()
    {
      SimpleList simpleList = SimpleList::new();
      numArray: Vec<i32> = new int[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      let mut mapWidth: i32 = self.game.Data.MapObj[0].MapWidth;
      for (let mut x: i32 = 0; x <= mapWidth; x += 1)
      {
        let mut mapHeight: i32 = self.game.Data.MapObj[0].MapHeight;
        for (let mut y: i32 = 0; y <= mapHeight; y += 1)
        {
          if (self.game.Data.MapObj[0].HexObj[x, y].LandscapeType == self.HIGHMOUNTAIN && self.game.HandyFunctionsObj.HasHexRoad(x, y, 0))
            self.game.Data.MapObj[0].HexObj[x, y].LandscapeType = self.LOWMOUNTAIN;
        }
      }
    }

    pub fn HarbourAssurance()
    {
      SimpleList simpleList = SimpleList::new();
      numArray: Vec<i32> = new int[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      if (self.opt4v == 100)
        return;
      let mut num1: i32 = 1;
      while (num1 == 1)
      {
        num1 = 0;
        self.game.AIObj.InitAIOnlyDim();
        self.game.AIObj.InitFindContinent();
        let mut continentCount: i32 = self.game.AIObj.ContinentCount;
        for (let mut index1: i32 = 1; index1 <= continentCount; index1 += 1)
        {
          let mut num2: i32 = 0;
          let mut num3: i32 = 0;
          let mut mapWidth1: i32 = self.game.Data.MapObj[0].MapWidth;
          Coordinate coordinate;
          for (let mut cx: i32 = 0; cx <= mapWidth1; cx += 1)
          {
            let mut mapHeight: i32 = self.game.Data.MapObj[0].MapHeight;
            for (let mut cy: i32 = 0; cy <= mapHeight; cy += 1)
            {
              if (self.game.Data.MapObj[0].HexObj[cx, cy].Location > -1 && self.game.AIObj.HexContinent[cx, cy] == index1)
              {
                let mut num4: i32 = 0;
                num2 += 1;
                let mut tfacing: i32 = 1;
                do
                {
                  coordinate = self.game.HandyFunctionsObj.HexNeighbourSameMap(cx, cy, 0, tfacing);
                  if (coordinate.onmap && self.game.Data.LandscapeTypeObj[self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].LandscapeType].IsSea)
                    num4 = 1;
                  tfacing += 1;
                }
                while (tfacing <= 6);
                if (num4 > 0)
                  num3 += 1;
              }
            }
          }
          if (num2 > 0 & (num3 == 0 |  num3 <  num2 / 3.0) & self.game.AIObj.ContinentCount > 1)
          {
            num1 = 1;
            let mut mapWidth2: i32 = self.game.Data.MapObj[0].MapWidth;
            for (let mut index2: i32 = 0; index2 <= mapWidth2; index2 += 1)
            {
              let mut mapHeight: i32 = self.game.Data.MapObj[0].MapHeight;
              for (let mut index3: i32 = 0; index3 <= mapHeight; index3 += 1)
                numArray[index2, index3] = 0;
            }
            let mut mapWidth3: i32 = self.game.Data.MapObj[0].MapWidth;
            for (let mut index4: i32 = 0; index4 <= mapWidth3; index4 += 1)
            {
              let mut mapHeight: i32 = self.game.Data.MapObj[0].MapHeight;
              for (let mut index5: i32 = 0; index5 <= mapHeight; index5 += 1)
              {
                if (self.game.Data.MapObj[0].HexObj[index4, index5].Location == -1 && self.game.AIObj.HexContinent[index4, index5] == index1)
                {
                  let mut num5: i32 = 0;
                  numArray[index4, index5] = 1;
                  let mut tfacing1: i32 = 1;
                  do
                  {
                    coordinate = self.game.HandyFunctionsObj.HexNeighbourSameMap(index4, index5, 0, tfacing1);
                    if (coordinate.onmap && numArray[coordinate.x, coordinate.y] == 0 && self.game.Data.LandscapeTypeObj[self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].LandscapeType].IsSea)
                      num5 = 1;
                    tfacing1 += 1;
                  }
                  while (tfacing1 <= 6);
                  let mut index6: i32 = 0;
                  do
                  {
                    if (self.game.Data.MapObj[0].HexObj[index4, index5].RoadType[index6] > -1)
                      num5 = 0;
                    index6 += 1;
                  }
                  while (index6 <= 5);
                  if (num5 > 0)
                  {
                    self.game.Data.MapObj[0].HexObj[index4, index5].LandscapeType =  Math.Round( self.game.Data.RuleVar[401]);
                    let mut tfacing2: i32 = 1;
                    do
                    {
                      coordinate = self.game.HandyFunctionsObj.HexNeighbourSameMap(index4, index5, 0, tfacing2);
                      if (coordinate.onmap)
                      {
                        self.game.Data.MapObj[0].HexObj[index4, index5].RiverType[tfacing2 - 1] = -1;
                        self.game.Data.MapObj[0].HexObj[index4, index5].RoadType[tfacing2 - 1] = -1;
                        self.game.Data.MapObj[0].HexObj[index4, index5].Bridge[tfacing2 - 1] = false;
                        let mut index7: i32 = self.game.HandyFunctionsObj.HexFacing(coordinate.x, coordinate.y, 0, index4, index5, 0) - 1;
                        self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].RiverType[index7] = -1;
                        self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].RoadType[index7] = -1;
                        self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].Bridge[index7] = false;
                      }
                      tfacing2 += 1;
                    }
                    while (tfacing2 <= 6);
                  }
                }
              }
            }
          }
        }
      }
    }

    pub fn MirrorTheMap()
    {
      SimpleList simpleList = SimpleList::new();
      int[] numArray1 = new int[6];
      int[] numArray2 = new int[10];
      let mut num1: i32 =  Math.Round(Conversion.Int( self.game.Data.MapObj[0].MapWidth / 2.0));
      let mut num2: i32 =  Math.Round(Conversion.Int( (self.game.Data.MapObj[0].MapHeight + 1) / 2.0));
      let mut num3: i32 = 3;
      let mut num4: i32 = num1;
      for (let mut index1: i32 = 0; index1 <= num4; index1 += 1)
      {
        let mut mapHeight: i32 = self.game.Data.MapObj[0].MapHeight;
        for (let mut index2: i32 = 0; index2 <= mapHeight; index2 += 1)
        {
          if (index1 != self.game.Data.MapObj[0].MapWidth - index1)
          {
            let mut index3: i32 = index2 < num2 ? (index2 >= num2 ? num2 : self.game.Data.MapObj[0].MapHeight - index2) : self.game.Data.MapObj[0].MapHeight - index2;
            let mut index4: i32 = 0;
            do
            {
              numArray2[index4] = self.game.Data.MapObj[0].HexObj[self.game.Data.MapObj[0].MapWidth - index1, index2].AreaCode[index4];
              index4 += 1;
            }
            while (index4 <= 9);
            self.game.Data.MapObj[0].HexObj[self.game.Data.MapObj[0].MapWidth - index1, index2] = self.game.Data.MapObj[0].HexObj[index1, index3].Clone();
            let mut index5: i32 = 0;
            do
            {
              self.game.Data.MapObj[0].HexObj[self.game.Data.MapObj[0].MapWidth - index1, index2].AreaCode[index5] = numArray2[index5];
              index5 += 1;
            }
            while (index5 <= 9);
            let mut index6: i32 = 0;
            do
            {
              numArray1[index6] = self.game.Data.MapObj[0].HexObj[self.game.Data.MapObj[0].MapWidth - index1, index2].RiverType[index6];
              index6 += 1;
            }
            while (index6 <= 5);
            let mut index7: i32 = 0;
            do
            {
              let mut index8: i32 = index7 + num3;
              if (index8 > 5)
                index8 -= 6;
              self.game.Data.MapObj[0].HexObj[self.game.Data.MapObj[0].MapWidth - index1, index2].RiverType[index7] = numArray1[index8];
              index7 += 1;
            }
            while (index7 <= 5);
          }
        }
      }
      let mut mapWidth: i32 = self.game.Data.MapObj[0].MapWidth;
      for (let mut index9: i32 = 0; index9 <= mapWidth; index9 += 1)
      {
        let mut mapHeight: i32 = self.game.Data.MapObj[0].MapHeight;
        for (let mut index10: i32 = 0; index10 <= mapHeight; index10 += 1)
        {
          let mut index11: i32 = 0;
          do
          {
            if (self.game.Data.MapObj[0].HexObj[index9, index10].RiverType[index11] > -1)
            {
              Coordinate coordinate = self.game.HandyFunctionsObj.HexNeighbourSameMap(index9, index10, 0, index11 + 1);
              if (coordinate.onmap)
              {
                let mut index12: i32 = self.game.HandyFunctionsObj.HexFacing(coordinate.x, coordinate.y, 0, index9, index10, 0) - 1;
                if (self.game.Data.MapObj[0].HexObj[index9, index10].RiverType[index11] != self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].RiverType[index12])
                  self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].RiverType[index12] = self.game.Data.MapObj[0].HexObj[index9, index10].RiverType[index11];
              }
            }
            index11 += 1;
          }
          while (index11 <= 5);
        }
      }
    }

    pub fn MakeRoads2(x: i32, y: i32, roads: i32)
    {
      let mut movetype: i32 =  Math.Round( self.game.Data.RuleVar[99]);
      if ( self.game.Data.RuleVar[478] > 0.0)
        movetype =  Math.Round( self.game.Data.RuleVar[478]);
      if (self.game.Data.LocTypeObj[self.game.Data.LocObj[self.game.Data.MapObj[0].HexObj[x, y].Location].Type].MaxProd < 2000)
        return;
      self.game.HandyFunctionsObj.MakeMovePrediction2(self.game.Data.MapObj[0].HexObj[x, y].Regime, movetype, 0, roads * 250, x, y, 0, false, muststartonairfield: false, NoAPPenalties: true, SeaBlock: true, BlockAllSea: true, NoBridgePenalty: true);
      SimpleList simpleList = SimpleList::new();
      let mut mapWidth: i32 = self.game.Data.MapObj[0].MapWidth;
      for (let mut index: i32 = 0; index <= mapWidth; index += 1)
      {
        let mut mapHeight: i32 = self.game.Data.MapObj[0].MapHeight;
        for (let mut tdata2: i32 = 0; tdata2 <= mapHeight; tdata2 += 1)
        {
          if (!(index == x & tdata2 == y) && self.game.Data.MapObj[0].HexObj[index, tdata2].Location > -1 && self.game.EditObj.TempValue[0].Value[index, tdata2] < 150 && self.game.Data.LocTypeObj[self.game.Data.LocObj[self.game.Data.MapObj[0].HexObj[index, tdata2].Location].Type].MaxProd >= 2000)
            simpleList.Add(index,  Math.Round( (VBMath.Rnd() *  self.game.EditObj.TempValue[0].Value[index, tdata2])), index, tdata2);
        }
      }
      if (simpleList.Counter <= -1)
        return;
      simpleList.Sort();
      let mut counter: i32 = simpleList.Counter;
      for (let mut index1: i32 = 0; index1 <= counter; index1 += 1)
      {
        if (index1 < roads + 15)
        {
          self.game.HandyFunctionsObj.MakeMovePrediction2(self.game.Data.MapObj[0].HexObj[x, y].Regime, movetype, 0, roads * 250, simpleList.Data1[index1], simpleList.Data2[index1], 0, false, muststartonairfield: false, SeaBlock: true, BlockAllSea: true, NoBridgePenalty: true, BridgeAP: 40);
          let mut index2: i32 = x;
          y1: i32;
          for (let mut index3: i32 = y; !(index2 == simpleList.Data1[index1] & index3 == simpleList.Data2[index1]); index3 = y1)
          {
            let mut tfacing1: i32 = -1;
            let mut num1: i32 = 9999;
            let mut tfacing2: i32 = 1;
            Coordinate coordinate;
            do
            {
              coordinate = self.game.HandyFunctionsObj.HexNeighbourSameMap(index2, index3, 0, tfacing2);
              if (coordinate.onmap && self.game.EditObj.TempValue[0].Value[coordinate.x, coordinate.y] < num1)
              {
                num1 = self.game.EditObj.TempValue[0].Value[coordinate.x, coordinate.y];
                tfacing1 = tfacing2;
              }
              tfacing2 += 1;
            }
            while (tfacing2 <= 6);
            coordinate = self.game.HandyFunctionsObj.HexNeighbourSameMap(index2, index3, 0, tfacing1);
            let mut index4: i32 = tfacing1 - 1;
            let mut index5: i32 = self.game.HandyFunctionsObj.HexFacing(coordinate.x, coordinate.y, 0, index2, index3, 0) - 1;
            self.game.Data.MapObj[0].HexObj[index2, index3].RoadType[index4] =  Math.Round( self.game.Data.RuleVar[475]);
            self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].RoadType[index5] =  Math.Round( self.game.Data.RuleVar[475]);
            if (self.game.Data.MapObj[0].HexObj[index2, index3].RiverType[index4] > -1)
            {
              self.game.Data.MapObj[0].HexObj[index2, index3].Bridge[index4] = true;
              self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].Bridge[index5] = true;
            }
            let mut x1: i32 = coordinate.x;
            y1 = coordinate.y;
            if (self.domirror == 1)
            {
              let mut num2: i32 =  Math.Round(Conversion.Int( (self.game.Data.MapObj[0].MapWidth + 1) / 2.0));
              let mut num3: i32 =  Math.Round(Conversion.Int( (self.game.Data.MapObj[0].MapHeight + 1) / 2.0));
              let mut index6: i32 = index3 < num3 ? (index3 >= num3 ? num3 : self.game.Data.MapObj[0].MapHeight - index3) : self.game.Data.MapObj[0].MapHeight - index3;
              let mut index7: i32 = index2 < num2 ? (index2 >= num2 ? num2 : self.game.Data.MapObj[0].MapWidth - index2) : self.game.Data.MapObj[0].MapWidth - index2;
              let mut tfacing3: i32 = index4 + 1 + 3;
              if (tfacing3 > 6)
                tfacing3 -= 6;
              coordinate = self.game.HandyFunctionsObj.HexNeighbourSameMap(index7, index6, 0, tfacing3);
              let mut index8: i32 = tfacing3 - 1;
              let mut index9: i32 = self.game.HandyFunctionsObj.HexFacing(coordinate.x, coordinate.y, 0, index7, index6, 0) - 1;
              self.game.Data.MapObj[0].HexObj[index7, index6].RoadType[index8] =  Math.Round( self.game.Data.RuleVar[475]);
              self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].RoadType[index9] =  Math.Round( self.game.Data.RuleVar[475]);
              if (self.game.Data.MapObj[0].HexObj[index7, index6].RiverType[index8] > -1)
              {
                self.game.Data.MapObj[0].HexObj[index7, index6].Bridge[index8] = true;
                self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].Bridge[index9] = true;
              }
            }
            index2 = x1;
          }
        }
      }
    }

    pub fn MakeRoads(x: i32, y: i32, roads: i32, bool secondroads, bool verysmall = false)
    {
      let mut movetype: i32 =  Math.Round( self.game.Data.RuleVar[99]);
      if ( self.game.Data.RuleVar[478] > 0.0)
        movetype =  Math.Round( self.game.Data.RuleVar[478]);
      if (verysmall)
        self.game.HandyFunctionsObj.MakeMovePrediction2(0, movetype, 0, 30, x, y, 0, false, false, muststartonairfield: false, NoAPPenalties: true, SeaBlock: true, BlockAllSea: true, NoBridgePenalty: true, BridgeAP: 10, MaxDistance: 4);
      else
        self.game.HandyFunctionsObj.MakeMovePrediction2(0, movetype, 0, roads * 250, x, y, 0, false, false, muststartonairfield: false, NoAPPenalties: true, SeaBlock: true, BlockAllSea: true, NoBridgePenalty: true, BridgeAP: 40, MaxDistance: 32);
      SimpleList simpleList = SimpleList::new();
      let mut MaxDistance: i32 = 9999;
      let mut num1: i32 = roads;
      if ( self.game.Data.RuleVar[474] == 1.0)
      {
        num1 = 99;
        MaxDistance = 15;
      }
      if (!secondroads)
      {
        num1 = roads;
        MaxDistance = 32;
      }
      let mut locCounter: i32 = self.game.Data.LocCounter;
      for (let mut index: i32 = 0; index <= locCounter; index += 1)
      {
        let mut x1: i32 = self.game.Data.LocObj[index].X;
        let mut y1: i32 = self.game.Data.LocObj[index].Y;
        if (!(x1 == x & y1 == y) && self.game.Data.MapObj[0].HexObj[x1, y1].VP > 0)
        {
          if (self.game.EditObj.TempValue[0].Value[x1, y1] < 600 & !secondroads)
            simpleList.Add(x1,  Math.Round( (VBMath.Rnd() *  self.game.EditObj.TempValue[0].Value[x1, y1])), x1, y1);
          if (self.game.EditObj.TempValue[0].Value[x1, y1] < 200 & secondroads)
            simpleList.Add(x1,  Math.Round( (VBMath.Rnd() *  self.game.EditObj.TempValue[0].Value[x1, y1])), x1, y1);
        }
      }
      if (simpleList.Counter <= -1)
        return;
      simpleList.Sort();
      let mut counter: i32 = simpleList.Counter;
      for (let mut index1: i32 = 0; index1 <= counter; index1 += 1)
      {
        if (index1 < num1)
        {
          if (!(index1 < roads & !secondroads) && index1 <= 10 & secondroads)
            self.game.HandyFunctionsObj.MakeMovePrediction2(self.game.Data.MapObj[0].HexObj[x, y].Regime, movetype, 0, roads * 350, simpleList.Data1[index1], simpleList.Data2[index1], 0, false, muststartonairfield: false, SeaBlock: true, BlockAllSea: true, NoBridgePenalty: true, overruleRoadType: 0, BridgeAP: 40, MaxDistance: MaxDistance);
          if (self.game.EditObj.TempValue[0].Value[x, y] < 9999 & secondroads | self.game.EditObj.TempValue[0].Value[simpleList.Data1[index1], simpleList.Data2[index1]] < 9999 & !secondroads)
          {
            let mut num2: i32 = 1;
            if (index1 >= roads)
            {
              if (secondroads)
              {
                let mut num3: i32 = self.game.EditObj.TempValue[0].Value[x, y];
                self.game.HandyFunctionsObj.MakeMovePrediction2(self.game.Data.MapObj[0].HexObj[x, y].Regime, movetype, 0, roads * 350, simpleList.Data1[index1], simpleList.Data2[index1], 0, false, SeaBlock: true, BlockAllSea: true, NoBridgePenalty: true, BridgeAP: 40, MaxDistance: MaxDistance);
                if (self.game.EditObj.TempValue[0].Value[x, y] < num3 + 18 | self.game.EditObj.TempValue[0].Value[x, y] > roads * 350)
                  num2 = 0;
                if (num2 == 1)
                  self.game.HandyFunctionsObj.MakeMovePrediction2(self.game.Data.MapObj[0].HexObj[x, y].Regime, movetype, 0, roads * 350, simpleList.Data1[index1], simpleList.Data2[index1], 0, false, SeaBlock: true, BlockAllSea: true, NoBridgePenalty: true, overruleRoadType: 0, BridgeAP: 40, MaxDistance: MaxDistance);
              }
              else
                num2 = 0;
            }
            Coordinate coordinate;
            if (num2 == 1)
            {
              if (secondroads)
              {
                let mut index2: i32 = x;
                y2: i32;
                for (let mut index3: i32 = y; !(index2 == simpleList.Data1[index1] & index3 == simpleList.Data2[index1]); index3 = y2)
                {
                  let mut tfacing1: i32 = -1;
                  let mut num4: i32 = 9999;
                  let mut tfacing2: i32 = 1;
                  do
                  {
                    coordinate = self.game.HandyFunctionsObj.HexNeighbourSameMap(index2, index3, 0, tfacing2);
                    if (coordinate.onmap && self.game.EditObj.TempValue[0].Value[coordinate.x, coordinate.y] < num4)
                    {
                      num4 = self.game.EditObj.TempValue[0].Value[coordinate.x, coordinate.y];
                      tfacing1 = tfacing2;
                    }
                    tfacing2 += 1;
                  }
                  while (tfacing2 <= 6);
                  coordinate = self.game.HandyFunctionsObj.HexNeighbourSameMap(index2, index3, 0, tfacing1);
                  let mut index4: i32 = tfacing1 - 1;
                  let mut index5: i32 = self.game.HandyFunctionsObj.HexFacing(coordinate.x, coordinate.y, 0, index2, index3, 0) - 1;
                  self.game.Data.MapObj[0].HexObj[index2, index3].RoadType[index4] =  Math.Round( self.game.Data.RuleVar[409]);
                  self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].RoadType[index5] =  Math.Round( self.game.Data.RuleVar[409]);
                  if (self.game.Data.MapObj[0].HexObj[index2, index3].RiverType[index4] > -1)
                  {
                    self.game.Data.MapObj[0].HexObj[index2, index3].Bridge[index4] = true;
                    self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].Bridge[index5] = true;
                  }
                  let mut x2: i32 = coordinate.x;
                  y2 = coordinate.y;
                  if (self.domirror == 1)
                  {
                    let mut num5: i32 =  Math.Round(Conversion.Int( (self.game.Data.MapObj[0].MapWidth + 1) / 2.0));
                    let mut num6: i32 =  Math.Round(Conversion.Int( (self.game.Data.MapObj[0].MapHeight + 1) / 2.0));
                    let mut index6: i32 = index3 < num6 ? (index3 >= num6 ? num6 : self.game.Data.MapObj[0].MapHeight - index3) : self.game.Data.MapObj[0].MapHeight - index3;
                    let mut index7: i32 = index2 < num5 ? (index2 >= num5 ? num5 : self.game.Data.MapObj[0].MapWidth - index2) : self.game.Data.MapObj[0].MapWidth - index2;
                    let mut tfacing3: i32 = index4 + 1 + 3;
                    if (tfacing3 > 6)
                      tfacing3 -= 6;
                    coordinate = self.game.HandyFunctionsObj.HexNeighbourSameMap(index7, index6, 0, tfacing3);
                    let mut index8: i32 = tfacing3 - 1;
                    let mut index9: i32 = self.game.HandyFunctionsObj.HexFacing(coordinate.x, coordinate.y, 0, index7, index6, 0) - 1;
                    self.game.Data.MapObj[0].HexObj[index7, index6].RoadType[index8] =  Math.Round( self.game.Data.RuleVar[409]);
                    self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].RoadType[index9] =  Math.Round( self.game.Data.RuleVar[409]);
                    if (self.game.Data.MapObj[0].HexObj[index7, index6].RiverType[index8] > -1)
                    {
                      self.game.Data.MapObj[0].HexObj[index7, index6].Bridge[index8] = true;
                      self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].Bridge[index9] = true;
                    }
                  }
                  index2 = x2;
                }
              }
              else
              {
                let mut index10: i32 = simpleList.Data1[index1];
                y3: i32;
                for (let mut index11: i32 = simpleList.Data2[index1]; !(index10 == x & index11 == y); index11 = y3)
                {
                  let mut tfacing4: i32 = -1;
                  let mut num7: i32 = 9999;
                  let mut tfacing5: i32 = 1;
                  do
                  {
                    coordinate = self.game.HandyFunctionsObj.HexNeighbourSameMap(index10, index11, 0, tfacing5);
                    if (coordinate.onmap && self.game.EditObj.TempValue[0].Value[coordinate.x, coordinate.y] < num7)
                    {
                      num7 = self.game.EditObj.TempValue[0].Value[coordinate.x, coordinate.y];
                      tfacing4 = tfacing5;
                    }
                    tfacing5 += 1;
                  }
                  while (tfacing5 <= 6);
                  coordinate = self.game.HandyFunctionsObj.HexNeighbourSameMap(index10, index11, 0, tfacing4);
                  let mut index12: i32 = tfacing4 - 1;
                  let mut index13: i32 = self.game.HandyFunctionsObj.HexFacing(coordinate.x, coordinate.y, 0, index10, index11, 0) - 1;
                  self.game.Data.MapObj[0].HexObj[index10, index11].RoadType[index12] =  Math.Round( self.game.Data.RuleVar[409]);
                  self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].RoadType[index13] =  Math.Round( self.game.Data.RuleVar[409]);
                  if (self.game.Data.MapObj[0].HexObj[index10, index11].RiverType[index12] > -1)
                  {
                    self.game.Data.MapObj[0].HexObj[index10, index11].Bridge[index12] = true;
                    self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].Bridge[index13] = true;
                  }
                  let mut x3: i32 = coordinate.x;
                  y3 = coordinate.y;
                  if (self.domirror == 1)
                  {
                    let mut num8: i32 =  Math.Round(Conversion.Int( (self.game.Data.MapObj[0].MapWidth + 1) / 2.0));
                    let mut num9: i32 =  Math.Round(Conversion.Int( (self.game.Data.MapObj[0].MapHeight + 1) / 2.0));
                    let mut index14: i32 = index11 < num9 ? (index11 >= num9 ? num9 : self.game.Data.MapObj[0].MapHeight - index11) : self.game.Data.MapObj[0].MapHeight - index11;
                    let mut index15: i32 = index10 < num8 ? (index10 >= num8 ? num8 : self.game.Data.MapObj[0].MapWidth - index10) : self.game.Data.MapObj[0].MapWidth - index10;
                    let mut tfacing6: i32 = index12 + 1 + 3;
                    if (tfacing6 > 6)
                      tfacing6 -= 6;
                    coordinate = self.game.HandyFunctionsObj.HexNeighbourSameMap(index15, index14, 0, tfacing6);
                    let mut index16: i32 = tfacing6 - 1;
                    let mut index17: i32 = self.game.HandyFunctionsObj.HexFacing(coordinate.x, coordinate.y, 0, index15, index14, 0) - 1;
                    self.game.Data.MapObj[0].HexObj[index15, index14].RoadType[index16] =  Math.Round( self.game.Data.RuleVar[409]);
                    self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].RoadType[index17] =  Math.Round( self.game.Data.RuleVar[409]);
                    if (self.game.Data.MapObj[0].HexObj[index15, index14].RiverType[index16] > -1)
                    {
                      self.game.Data.MapObj[0].HexObj[index15, index14].Bridge[index16] = true;
                      self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].Bridge[index17] = true;
                    }
                  }
                  index10 = x3;
                }
              }
            }
            self.game.HandyFunctionsObj.RedimTempValue(9999);
          }
        }
      }
    }

    pub fn PlaceRegimes(x: i32, y: i32, regmax: i32)
    {
      let mut num1: i32 = -1;
      if (self.doblockcenter == 1)
      {
        self.tempcount = 1;
        self.tempx[1] =  Math.Round(Conversion.Int( self.game.Data.MapObj[0].MapWidth / 2.0));
        self.tempy[1] =  Math.Round(Conversion.Int( self.game.Data.MapObj[0].MapHeight / 2.0));
      }
      let mut num2: i32 = regmax;
      for (let mut index1: i32 = 1; index1 <= num2; index1 += 1)
      {
        let mut num3: i32 = 0;
        let mut predef: i32 = 0;
        while (num3 == 0)
        {
          let mut num4: i32 = 0;
          let mut num5: i32 = 0;
          let mut num6: i32 = 0;
          let mut num7: i32 = 0;
          Coordinate coordinate;
          while (num7 < 1000)
          {
            let mut index2: i32 =  Math.Round( (Conversion.Int(VBMath.Rnd() *  (x - 5)) + 3f));
            let mut index3: i32 =  Math.Round( (Conversion.Int(VBMath.Rnd() *  (y - 5)) + 3f));
            num7 += 1;
            if (self.game.Data.MapObj[0].HexObj[index2, index3].LandscapeType == self.GRASS | self.game.Data.MapObj[0].HexObj[index2, index3].LandscapeType == self.LIGHTFOREST | predef > 10 && self.game.Data.MapObj[0].HexObj[index2, index3].Location == -1)
            {
              if (self.tempcount == 0)
              {
                num4 = index2;
                num5 = index3;
              }
              else
              {
                let mut num8: i32 = 999;
                let mut tempcount: i32 = self.tempcount;
                for (let mut index4: i32 = 0; index4 <= tempcount; index4 += 1)
                {
                  let mut num9: i32 = self.game.HandyFunctionsObj.Distance(index2, index3, 0, self.tempx[index4], self.tempy[index4], 0);
                  if (self.game.Data.MapObj[0].HexObj[self.tempx[index4], self.tempy[index4]].Regime > -1)
                    num9 =  Math.Round(Conversion.Int( num9 / 3.0));
                  if (num9 < num8)
                    num8 = num9;
                }
                if ( self.game.Data.RuleVar[481] > 0.0)
                {
                  if (self.game.Data.MapObj[0].HexObj[index2, index3].AreaCode[ Math.Round( self.game.Data.RuleVar[481])] == self.RegFavClimate[index1 - 1] & self.RegFavClimate[index1 - 1] > 0)
                  {
                    num8 *= 20;
                  }
                  else
                  {
                    if (self.game.Data.MapObj[0].HexObj[index2, index3].AreaCode[ Math.Round( self.game.Data.RuleVar[481])] == 99)
                      num8 = -1;
                    if (self.game.Data.MapObj[0].HexObj[index2, index3].AreaCode[ Math.Round( self.game.Data.RuleVar[481])] == 1)
                      num8 = -1;
                  }
                }
                let mut num10: i32 = 0;
                let mut tfacing: i32 = 1;
                do
                {
                  coordinate = self.game.HandyFunctionsObj.HexNeighbour(index2, index3, 0, tfacing);
                  if (coordinate.onmap && self.game.HandyFunctionsObj.IsHexHarbourOrSea(coordinate.x, coordinate.y, coordinate.map))
                    num10 += 1;
                  tfacing += 1;
                }
                while (tfacing <= 6);
                if (num10 >= 5)
                  num8 =  Math.Round( num8 / 40.0);
                if (num8 > num6)
                {
                  num6 = num8;
                  num4 = index2;
                  num5 = index3;
                }
              }
            }
          }
          let mut index5: i32 = num4;
          let mut index6: i32 = num5;
          predef += 1;
          num11: i32;
          if (self.domirror == 1)
          {
            if (num1 == -1)
            {
              let mut num12: i32 =  Math.Round(Conversion.Int( (self.game.Data.MapObj[0].MapWidth + 1) / 2.0));
              let mut num13: i32 =  Math.Round(Conversion.Int( (self.game.Data.MapObj[0].MapHeight + 1) / 2.0));
              let mut num14: i32 = index6 < num13 ? (index6 >= num13 ? num13 : self.game.Data.MapObj[0].MapHeight - index6) : self.game.Data.MapObj[0].MapHeight - index6;
              num1 = index5 < num12 ? (index5 >= num12 ? num12 : self.game.Data.MapObj[0].MapWidth - index5) : self.game.Data.MapObj[0].MapWidth - index5;
              num11 = num14;
            }
            else
            {
              index5 = num1;
              index6 = num11;
              num1 = -1;
              num11 = -2;
            }
          }
          if (predef > 10 | num11 == -2 & self.domirror == 1 | self.game.Data.MapObj[0].HexObj[index5, index6].LandscapeType == self.GRASS | self.game.Data.MapObj[0].HexObj[index5, index6].LandscapeType == self.LIGHTFOREST && self.game.Data.MapObj[0].HexObj[index5, index6].Location == -1)
          {
            this += 1.tempcount;
            self.tempx[self.tempcount] = index5;
            self.tempy[self.tempcount] = index6;
            self.game.Data.AddLoc(index5, index6, 0);
            let mut locCounter: i32 = self.game.Data.LocCounter;
            self.game.Data.LocObj[locCounter].People = 0;
            if ((self.Regid[index1 - 1] + 22) % 7 == 1)
            {
              self.game.Data.LocObj[locCounter].People =  Math.Round( self.game.Data.RuleVar[458]);
              self.game.Data.RegimeObj[index1 - 1].People =  Math.Round( self.game.Data.RuleVar[458]);
              if ( self.game.Data.RuleVar[422] > 0.0)
                self.game.Data.RegimeObj[index1 - 1].OfficerPool =  Math.Round( self.game.Data.RuleVar[422]);
            }
            else if ((self.Regid[index1 - 1] + 22) % 7 == 2)
            {
              self.game.Data.LocObj[locCounter].People =  Math.Round( self.game.Data.RuleVar[459]);
              self.game.Data.RegimeObj[index1 - 1].People =  Math.Round( self.game.Data.RuleVar[459]);
              if ( self.game.Data.RuleVar[422] > 0.0)
                self.game.Data.RegimeObj[index1 - 1].OfficerPool =  Math.Round( self.game.Data.RuleVar[427]);
            }
            else if ((self.Regid[index1 - 1] + 22) % 7 == 3)
            {
              self.game.Data.LocObj[locCounter].People =  Math.Round( self.game.Data.RuleVar[460]);
              self.game.Data.RegimeObj[index1 - 1].People =  Math.Round( self.game.Data.RuleVar[460]);
              if ( self.game.Data.RuleVar[422] > 0.0)
                self.game.Data.RegimeObj[index1 - 1].OfficerPool =  Math.Round( self.game.Data.RuleVar[432]);
            }
            else if ((self.Regid[index1 - 1] + 22) % 7 == 4)
            {
              self.game.Data.LocObj[locCounter].People =  Math.Round( self.game.Data.RuleVar[469]);
              self.game.Data.RegimeObj[index1 - 1].People =  Math.Round( self.game.Data.RuleVar[469]);
              if ( self.game.Data.RuleVar[422] > 0.0)
                self.game.Data.RegimeObj[index1 - 1].OfficerPool =  Math.Round( self.game.Data.RuleVar[466]);
            }
            else if ((self.Regid[index1 - 1] + 22) % 7 == 5)
            {
              self.game.Data.LocObj[locCounter].People =  Math.Round( self.game.Data.RuleVar[485]);
              self.game.Data.RegimeObj[index1 - 1].People =  Math.Round( self.game.Data.RuleVar[485]);
              if ( self.game.Data.RuleVar[422] > 0.0)
                self.game.Data.RegimeObj[index1 - 1].OfficerPool =  Math.Round( self.game.Data.RuleVar[482]);
            }
            else if ((self.Regid[index1 - 1] + 22) % 7 == 6)
            {
              self.game.Data.LocObj[locCounter].People =  Math.Round( self.game.Data.RuleVar[489]);
              self.game.Data.RegimeObj[index1 - 1].People =  Math.Round( self.game.Data.RuleVar[489]);
              if ( self.game.Data.RuleVar[422] > 0.0)
                self.game.Data.RegimeObj[index1 - 1].OfficerPool =  Math.Round( self.game.Data.RuleVar[486]);
            }
            else
            {
              self.game.Data.LocObj[locCounter].People =  Math.Round( self.game.Data.RuleVar[493]);
              self.game.Data.RegimeObj[index1 - 1].People =  Math.Round( self.game.Data.RuleVar[493]);
              if ( self.game.Data.RuleVar[422] > 0.0)
                self.game.Data.RegimeObj[index1 - 1].OfficerPool =  Math.Round( self.game.Data.RuleVar[490]);
            }
            self.game.Data.LocObj[locCounter].Name = "temp";
            let mut index7: i32 =  Math.Round( self.game.Data.RuleVar[410]);
            self.game.Data.LocObj[locCounter].Type = index7;
            if (self.game.Data.LocTypeObj[index7].AutoProd > -1)
            {
              self.game.Data.LocObj[locCounter].ProdPercent[0] = 100;
              self.game.Data.LocObj[locCounter].Production[0] = self.game.Data.LocTypeObj[index7].AutoProd;
            }
            self.TownSize[locCounter] = 3;
            self.TownCapitol[locCounter] = true;
            self.game.Data.MapObj[0].HexObj[index5, index6].VP = 4;
            self.totvp += 4;
            self.game.Data.LocObj[locCounter].StructuralPts = self.game.Data.LocTypeObj[index7].StructuralPts;
            self.game.Data.MapObj[0].HexObj[index5, index6].Location = locCounter;
            if (self.game.Data.LocTypeObj[index7].OverdrawLTNr > -1)
            {
              self.game.Data.MapObj[0].HexObj[index5, index6].LandscapeType = self.game.Data.LocTypeObj[index7].OverdrawLTNr;
              self.game.Data.MapObj[0].HexObj[index5, index6].SpriteNr = self.game.Data.LocTypeObj[index7].OverdrawSpriteNr;
            }
            self.game.Data.MapObj[0].HexObj[index5, index6].Regime = index1 - 1;
            let mut unr: i32 = self.game.Data.AddUnit(index5, index6, 0);
            self.game.Data.UnitObj[unr].Name = "Supreme HQ";
            if ( self.game.Data.RuleVar[343] == 1.0)
            {
              self.game.Data.AddHistoricalUnit();
              self.game.Data.UnitObj[unr].Historical = self.game.Data.HistoricalUnitCounter;
              self.game.Data.HistoricalUnitObj[self.game.Data.HistoricalUnitCounter].TempRegime = index1 - 1;
              self.game.ProcessingObj.RecruitOfficer(index1 - 1, false, self.game.Data.HistoricalUnitCounter);
            }
            self.game.Data.UnitObj[unr].Regime = index1 - 1;
            self.game.Data.UnitObj[unr].Supply = 500;
            self.game.Data.UnitObj[unr].SOSupReqPercent = 100;
            self.game.Data.UnitObj[unr].IsHQ = true;
            self.game.Data.LocObj[locCounter].HQ = unr;
            let mut index8: i32 = self.game.Data.AddSF(unr);
            self.game.Data.SFObj[index8].Type =  Math.Round( self.game.Data.RuleVar[411]);
            self.game.Data.SFObj[index8].Qty =  Math.Round( self.game.Data.RuleVar[412]);
            self.game.Data.SFObj[index8].Rdn = 100;
            self.game.Data.SFObj[index8].People = self.game.Data.RegimeObj[index1 - 1].People;
            self.game.Data.SFObj[index8].Xp = 25;
            self.game.Data.SFObj[index8].Mor = 50;
            if ( self.game.Data.RuleVar[476] > 0.0)
            {
              let mut index9: i32 = self.game.Data.AddSF(unr);
              self.game.Data.SFObj[index9].Type =  Math.Round( self.game.Data.RuleVar[476]);
              self.game.Data.SFObj[index9].Qty =  Math.Round( self.game.Data.RuleVar[477]);
              self.game.Data.SFObj[index9].Rdn = 100;
              self.game.Data.SFObj[index9].People = 0;
              self.game.Data.SFObj[index9].Xp = 25;
              self.game.Data.SFObj[index9].Mor = 50;
            }
            num3 = 1;
            let mut num15: i32 = 0;
            do
            {
              if ( self.game.Data.RuleVar[453 + num15] > 0.0)
              {
                predef =  Math.Round( self.game.Data.RuleVar[453 + num15]);
                self.game.EventRelatedObj.ExecAddUnit(predef, index5, index6, index1 - 1, "");
              }
              num15 += 1;
            }
            while (num15 <= 3);
            let mut tfacing: i32 = 1;
            do
            {
              coordinate = self.game.HandyFunctionsObj.HexNeighbourSameMap(index5, index6, 0, tfacing);
              if (coordinate.onmap && !self.game.Data.LandscapeTypeObj[self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].LandscapeType].IsSea)
                self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].Regime = index1 - 1;
              tfacing += 1;
            }
            while (tfacing <= 6);
          }
        }
      }
    }

    pub fn PlaceRegimes2()
    {
      if ( self.game.Data.RuleVar[461] != 1.0)
        return;
      SimpleList[] simpleListArray = new SimpleList[self.game.Data.RegimeCounter + 1];
      int[] numArray1 = new int[self.game.Data.RegimeCounter + 1];
      bool[] flagArray = new bool[self.game.Data.LocCounter + 1];
      let mut num1: i32 = 1;
      let mut regimeCounter: i32 = self.game.Data.RegimeCounter;
      if ( self.game.Data.RuleVar[496] > 0.0)
        --regimeCounter;
      while (num1 == 1)
      {
        num1 = 0;
        let mut num2: i32 = regimeCounter;
        for (let mut index1: i32 = 0; index1 <= num2; index1 += 1)
        {
          simpleListArray[index1] = SimpleList::new();
          let mut locCounter1: i32 = self.game.Data.LocCounter;
          for (let mut tid: i32 = 0; tid <= locCounter1; tid += 1)
          {
            if (self.game.Data.MapObj[0].HexObj[self.game.Data.LocObj[tid].X, self.game.Data.LocObj[tid].Y].Regime == -1)
            {
              let mut tweight: i32 = 9999;
              let mut num3: i32 = -1;
              let mut locCounter2: i32 = self.game.Data.LocCounter;
              for (let mut index2: i32 = 0; index2 <= locCounter2; index2 += 1)
              {
                if (self.game.Data.MapObj[0].HexObj[self.game.Data.LocObj[index2].X, self.game.Data.LocObj[index2].Y].Regime == index1)
                {
                  let mut num4: i32 = self.game.HandyFunctionsObj.Distance(self.game.Data.LocObj[tid].X, self.game.Data.LocObj[tid].Y, 0, self.game.Data.LocObj[index2].X, self.game.Data.LocObj[index2].Y, 0, 999);
                  if (num4 < tweight)
                  {
                    tweight = num4;
                    num3 = index2;
                  }
                }
              }
              if (num3 > -1)
                simpleListArray[index1].Add(tid, tweight);
            }
          }
        }
        let mut num5: i32 = 0;
        let mut num6: i32 = regimeCounter - 1;
        for (let mut index: i32 = 0; index <= num6; index += 1)
        {
          if (numArray1[index] != numArray1[index + 1])
            num5 = 1;
        }
        num7: i32;
        if (num5 == 0)
          num7 += 1;
        let mut num8: i32 = 0;
        let mut num9: i32 = regimeCounter;
        for (let mut index3: i32 = 0; index3 <= num9; index3 += 1)
        {
          if (simpleListArray[index3].Counter > -1 & numArray1[index3] < num7)
          {
            simpleListArray[index3].Sort();
            let mut counter: i32 = simpleListArray[index3].Counter;
            for (let mut index4: i32 = 0; index4 <= counter; index4 += 1)
            {
              let mut tid: i32 = simpleListArray[index3].Id[index4];
              if (self.game.Data.MapObj[0].HexObj[self.game.Data.LocObj[tid].X, self.game.Data.LocObj[tid].Y].Regime == -1 & !flagArray[tid])
              {
                self.game.Data.MapObj[0].HexObj[self.game.Data.LocObj[tid].X, self.game.Data.LocObj[tid].Y].Regime = index3;
                num1 = 1;
                flagArray[tid] = true;
                int[] numArray2 = numArray1;
                int[] numArray3 = numArray2;
                let mut index5: i32 = index3;
                let mut index6: i32 = index5;
                let mut num10: i32 = numArray2[index5] + self.game.Data.LocTypeObj[self.game.Data.LocObj[tid].Type].MaxProd;
                numArray3[index6] = num10;
                if ((self.Regid[index3] + 22) % 7 == 1)
                  self.game.Data.LocObj[tid].People =  Math.Round( self.game.Data.RuleVar[458]);
                else if ((self.Regid[index3] + 22) % 7 == 2)
                  self.game.Data.LocObj[tid].People =  Math.Round( self.game.Data.RuleVar[459]);
                else if ((self.Regid[index3] + 22) % 7 == 3)
                  self.game.Data.LocObj[tid].People =  Math.Round( self.game.Data.RuleVar[460]);
                else if ((self.Regid[index3] + 22) % 7 == 4)
                  self.game.Data.LocObj[tid].People =  Math.Round( self.game.Data.RuleVar[469]);
                else if ((self.Regid[index3] + 22) % 7 == 5)
                  self.game.Data.LocObj[tid].People =  Math.Round( self.game.Data.RuleVar[485]);
                else if ((self.Regid[index3] + 22) % 7 == 6)
                  self.game.Data.LocObj[tid].People =  Math.Round( self.game.Data.RuleVar[489]);
                else
                  self.game.Data.LocObj[tid].People =  Math.Round( self.game.Data.RuleVar[493]);
                if (numArray1[index3] > num7)
                  num7 = numArray1[index3];
                let mut num11: i32 = regimeCounter;
                for (let mut index7: i32 = 0; index7 <= num11; index7 += 1)
                  simpleListArray[index7].Remove(tid);
                num8 = 1;
                break;
              }
            }
          }
        }
      }
      self.town = new object[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      self.town2 = new object[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      let mut mapWidth1: i32 = self.game.Data.MapObj[0].MapWidth;
      for (let mut index8: i32 = 0; index8 <= mapWidth1; index8 += 1)
      {
        let mut mapHeight: i32 = self.game.Data.MapObj[0].MapHeight;
        for (let mut index9: i32 = 0; index9 <= mapHeight; index9 += 1)
        {
          self.town2[index8, index9] =  0;
          self.town[index8, index9] =  -1;
          if (self.game.Data.MapObj[0].HexObj[index8, index9].Regime > -1)
          {
            self.town[index8, index9] =  self.game.Data.MapObj[0].HexObj[index8, index9].Regime;
            self.town2[index8, index9] =  1;
          }
        }
      }
      let mut Right: i32 = 0;
      let mut num12: i32 = 10;
      while (num12 > 0)
      {
        Right += 1;
        --num12;
        let mut mapWidth2: i32 = self.game.Data.MapObj[0].MapWidth;
        for (let mut cx: i32 = 0; cx <= mapWidth2; cx += 1)
        {
          let mut mapHeight: i32 = self.game.Data.MapObj[0].MapHeight;
          for (let mut cy: i32 = 0; cy <= mapHeight; cy += 1)
          {
            if (Operators.ConditionalCompareObjectEqual(self.town2[cx, cy],  Right, false))
            {
              let mut tfacing: i32 = 1;
              do
              {
                Coordinate coordinate = self.game.HandyFunctionsObj.HexNeighbour(cx, cy, 0, tfacing);
                if (coordinate.onmap && Conversions.ToBoolean(Operators.OrObject(Operators.CompareObjectEqual(self.town[coordinate.x, coordinate.y],  -1, false), Operators.CompareObjectEqual(self.town[coordinate.x, coordinate.y], self.town[cx, cy], false))))
                {
                  if (self.game.Data.LandscapeTypeObj[self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].LandscapeType].IsSea)
                  {
                    if (Conversions.ToBoolean(Operators.OrObject(Operators.CompareObjectLess( (Right + 6), self.town2[coordinate.x, coordinate.y], false), Operators.CompareObjectEqual(self.town2[coordinate.x, coordinate.y],  0, false))))
                    {
                      self.town2[coordinate.x, coordinate.y] =  (Right + 6);
                      self.town[coordinate.x, coordinate.y] = RuntimeHelpers.GetObjectValue(self.town[cx, cy]);
                      num12 = 10;
                    }
                  }
                  else if (self.game.Data.MapObj[0].HexObj[cx, cy].RiverType[tfacing - 1] > -1)
                  {
                    if (Conversions.ToBoolean(Operators.OrObject(Operators.CompareObjectLess( (Right + 3), self.town2[coordinate.x, coordinate.y], false), Operators.CompareObjectEqual(self.town2[coordinate.x, coordinate.y],  0, false))))
                    {
                      self.town2[coordinate.x, coordinate.y] =  (Right + 3);
                      self.town[coordinate.x, coordinate.y] = RuntimeHelpers.GetObjectValue(self.town[cx, cy]);
                      num12 = 10;
                    }
                  }
                  else if (Conversions.ToBoolean(Operators.OrObject(Operators.CompareObjectLess( (Right + 1), self.town2[coordinate.x, coordinate.y], false), Operators.CompareObjectEqual(self.town2[coordinate.x, coordinate.y],  0, false))))
                  {
                    self.town2[coordinate.x, coordinate.y] =  (Right + 1);
                    self.town[coordinate.x, coordinate.y] = RuntimeHelpers.GetObjectValue(self.town[cx, cy]);
                    num12 = 10;
                  }
                }
                tfacing += 1;
              }
              while (tfacing <= 6);
            }
          }
        }
      }
      let mut mapWidth3: i32 = self.game.Data.MapObj[0].MapWidth;
      for (let mut index10: i32 = 0; index10 <= mapWidth3; index10 += 1)
      {
        let mut mapHeight: i32 = self.game.Data.MapObj[0].MapHeight;
        for (let mut index11: i32 = 0; index11 <= mapHeight; index11 += 1)
        {
          if (!self.game.Data.LandscapeTypeObj[self.game.Data.MapObj[0].HexObj[index10, index11].LandscapeType].IsSea)
            self.game.Data.MapObj[0].HexObj[index10, index11].Regime = Conversions.ToInteger(self.town[index10, index11]);
        }
      }
    }

    pub fn PlaceRegimes3()
    {
      let mut mapWidth1: i32 = self.game.Data.MapObj[0].MapWidth;
      for (let mut x: i32 = 0; x <= mapWidth1; x += 1)
      {
        let mut mapHeight: i32 = self.game.Data.MapObj[0].MapHeight;
        for (let mut y: i32 = 0; y <= mapHeight; y += 1)
        {
          if (self.game.Data.MapObj[0].HexObj[x, y].Location > -1 | self.game.Data.MapObj[0].HexObj[x, y].LandscapeType == self.LIGHTURBAN)
          {
            if ( self.game.Data.RuleVar[463] > 0.0)
              self.game.Data.MapObj[0].HexObj[x, y].AreaCode[ Math.Round( self.game.Data.RuleVar[463])] = -1;
            if (!self.game.Data.LandscapeTypeObj[self.game.Data.MapObj[0].HexObj[x, y].LandscapeType].IsSea)
            {
              let mut regnr: i32 = self.game.Data.MapObj[0].HexObj[x, y].Regime;
              if ( self.game.Data.RuleVar[461] == 1.0 && regnr == -1)
                regnr = Conversions.ToInteger(self.town[x, y]);
              if ( self.game.Data.RuleVar[463] > 0.0 & regnr > -1)
              {
                if ((self.Regid[regnr] + 22) % 7 == 1)
                  self.game.Data.MapObj[0].HexObj[x, y].AreaCode[ Math.Round( self.game.Data.RuleVar[463])] =  Math.Round( self.game.Data.RuleVar[458]);
                else if ((self.Regid[regnr] + 22) % 7 == 2)
                  self.game.Data.MapObj[0].HexObj[x, y].AreaCode[ Math.Round( self.game.Data.RuleVar[463])] =  Math.Round( self.game.Data.RuleVar[459]);
                else if ((self.Regid[regnr] + 22) % 7 == 3)
                  self.game.Data.MapObj[0].HexObj[x, y].AreaCode[ Math.Round( self.game.Data.RuleVar[463])] =  Math.Round( self.game.Data.RuleVar[460]);
                else if ((self.Regid[regnr] + 22) % 7 == 4)
                  self.game.Data.MapObj[0].HexObj[x, y].AreaCode[ Math.Round( self.game.Data.RuleVar[463])] =  Math.Round( self.game.Data.RuleVar[469]);
                else if ((self.Regid[regnr] + 22) % 7 == 5)
                  self.game.Data.MapObj[0].HexObj[x, y].AreaCode[ Math.Round( self.game.Data.RuleVar[463])] =  Math.Round( self.game.Data.RuleVar[485]);
                else if ((self.Regid[regnr] + 22) % 7 == 6)
                  self.game.Data.MapObj[0].HexObj[x, y].AreaCode[ Math.Round( self.game.Data.RuleVar[463])] =  Math.Round( self.game.Data.RuleVar[489]);
                else
                  self.game.Data.MapObj[0].HexObj[x, y].AreaCode[ Math.Round( self.game.Data.RuleVar[463])] =  Math.Round( self.game.Data.RuleVar[493]);
              }
              let mut location: i32 = self.game.Data.MapObj[0].HexObj[x, y].Location;
              if (location > -1 & regnr > -1)
              {
                if ((self.Regid[regnr] + 22) % 7 == 1 & self.game.Data.RegimeCounter == regnr &  self.game.Data.RuleVar[496] >= 1.0)
                  self.game.Data.LocObj[location].People =  Math.Round( self.game.Data.RuleVar[497]);
                else if ((self.Regid[regnr] + 22) % 7 == 1)
                  self.game.Data.LocObj[location].People =  Math.Round( self.game.Data.RuleVar[458]);
                else if ((self.Regid[regnr] + 22) % 7 == 2)
                  self.game.Data.LocObj[location].People =  Math.Round( self.game.Data.RuleVar[459]);
                else if ((self.Regid[regnr] + 22) % 7 == 3)
                  self.game.Data.LocObj[location].People =  Math.Round( self.game.Data.RuleVar[460]);
                else if ((self.Regid[regnr] + 22) % 7 == 4)
                  self.game.Data.LocObj[location].People =  Math.Round( self.game.Data.RuleVar[469]);
                else if ((self.Regid[regnr] + 22) % 7 == 5)
                  self.game.Data.LocObj[location].People =  Math.Round( self.game.Data.RuleVar[485]);
                else if ((self.Regid[regnr] + 22) % 7 == 6)
                  self.game.Data.LocObj[location].People =  Math.Round( self.game.Data.RuleVar[489]);
                else
                  self.game.Data.LocObj[location].People =  Math.Round( self.game.Data.RuleVar[493]);
              }
              if (location > -1)
                self.game.Data.LocObj[location].Name = self.GetRandomName(self.TownSize[location], self.game.Data.LocObj[location].People, self.TownCapitol[location]);
              if (location > -1 & regnr > -1 && self.dosinglestart == 0 && self.game.Data.MapObj[0].HexObj[x, y].Regime > -1 &  self.game.Data.RuleVar[462] > 0.0)
              {
                self.game.EventRelatedObj.ExecAddUnit( Math.Round( self.game.Data.RuleVar[462]), x, y, self.game.Data.MapObj[0].HexObj[x, y].Regime, self.game.Data.LocObj[location].Name + " Garrison");
                if ( self.game.Data.RuleVar[470] > 0.0)
                  self.game.EventRelatedObj.ExecAddUnit( Math.Round( self.game.Data.RuleVar[470]), x, y, self.game.Data.MapObj[0].HexObj[x, y].Regime, self.game.Data.LocObj[location].Name + " Garrison");
                if ( self.game.Data.RuleVar[471] > 0.0)
                  self.game.EventRelatedObj.ExecAddUnit( Math.Round( self.game.Data.RuleVar[471]), x, y, self.game.Data.MapObj[0].HexObj[x, y].Regime, self.game.Data.LocObj[location].Name + " Garrison");
                if ( self.game.Data.RuleVar[472] > 0.0)
                  self.game.EventRelatedObj.ExecAddUnit( Math.Round( self.game.Data.RuleVar[472]), x, y, self.game.Data.MapObj[0].HexObj[x, y].Regime, self.game.Data.LocObj[location].Name + " Garrison");
                self.game.HandyFunctionsObj.MakeMovePrediction2(self.game.Data.MapObj[0].HexObj[x, y].Regime,  Math.Round( self.game.Data.RuleVar[99]), 99,  Math.Round( self.game.Data.RuleVar[3]), x, y, 0);
                let mut num1: i32 = -1;
                let mut num2: i32 = 9999;
                let mut unitCounter: i32 = self.game.Data.UnitCounter;
                for (let mut index: i32 = 0; index <= unitCounter; index += 1)
                {
                  if (self.game.Data.UnitObj[index].IsHQ & self.game.Data.UnitObj[index].PreDef == -1 & self.game.Data.UnitObj[index].Regime == regnr &&  self.game.EditObj.TempValue[0].Value[self.game.Data.UnitObj[index].X, self.game.Data.UnitObj[index].Y] <=  self.game.Data.RuleVar[51] && self.game.EditObj.TempValue[0].Value[self.game.Data.UnitObj[index].X, self.game.Data.UnitObj[index].Y] < num2)
                  {
                    num2 = self.game.EditObj.TempValue[0].Value[self.game.Data.UnitObj[index].X, self.game.Data.UnitObj[index].Y];
                    num1 = index;
                  }
                }
                let mut hq: i32 = self.game.Data.UnitObj[self.game.Data.UnitCounter].HQ;
                if (num1 == -1)
                {
                  let mut unr: i32 = self.game.Data.AddUnit(x, y, 0);
                  self.game.Data.UnitObj[unr].Name = self.game.Data.LocObj[location].Name + " HQ";
                  if ( self.game.Data.RuleVar[343] == 1.0)
                  {
                    self.game.Data.AddHistoricalUnit();
                    self.game.Data.UnitObj[unr].Historical = self.game.Data.HistoricalUnitCounter;
                    self.game.Data.HistoricalUnitObj[self.game.Data.HistoricalUnitCounter].TempRegime = regnr - 1;
                    self.game.ProcessingObj.RecruitOfficer(regnr, false, self.game.Data.HistoricalUnitCounter);
                  }
                  self.game.Data.UnitObj[unr].Regime = regnr;
                  self.game.Data.UnitObj[unr].Supply = 50;
                  self.game.Data.UnitObj[unr].SOSupReqPercent = 100;
                  self.game.Data.UnitObj[unr].IsHQ = true;
                  self.game.Data.UnitObj[unr].HQ = hq;
                  self.game.Data.LocObj[location].HQ = unr;
                  let mut index: i32 = self.game.Data.AddSF(unr);
                  self.game.Data.SFObj[index].Type =  Math.Round( self.game.Data.RuleVar[411]);
                  self.game.Data.SFObj[index].Qty =  Math.Round( self.game.Data.RuleVar[412]);
                  self.game.Data.SFObj[index].Rdn = 100;
                  self.game.Data.SFObj[index].People = self.game.Data.RegimeObj[regnr].People;
                  self.game.Data.SFObj[index].Xp = 25;
                  self.game.Data.SFObj[index].Mor = 50;
                }
                else
                  self.game.Data.LocObj[location].HQ = num1;
              }
            }
          }
        }
      }
      if (self.dosinglestart == 0)
      {
        let mut unitCounter1: i32 = self.game.Data.UnitCounter;
        for (let mut hq1: i32 = 0; hq1 <= unitCounter1; hq1 += 1)
        {
          if (self.game.Data.UnitObj[hq1].PreDef == -1)
          {
            let mut x: i32 = self.game.Data.UnitObj[hq1].X;
            let mut y: i32 = self.game.Data.UnitObj[hq1].Y;
            let mut hq2: i32 = self.game.Data.UnitObj[hq1].HQ;
            let mut sfCount: i32 = self.game.Data.UnitObj[hq1].SFCount;
            for (let mut index: i32 = 0; index <= sfCount; index += 1)
              self.game.Data.SFObj[self.game.Data.UnitObj[hq1].SFList[index]].People = self.game.Data.RegimeObj[self.game.Data.UnitObj[hq1].Regime].People;
            if (hq2 > -1)
            {
              self.game.HandyFunctionsObj.MakeMovePrediction2(self.game.Data.MapObj[0].HexObj[x, y].Regime,  Math.Round( self.game.Data.RuleVar[0]), 99,  Math.Round( self.game.Data.RuleVar[3]), x, y, 0);
              let mut unitCounter2: i32 = self.game.Data.UnitCounter;
              for (let mut unr: i32 = 0; unr <= unitCounter2; unr += 1)
              {
                if (unr != hq1 && !self.game.HandyFunctionsObj.IsUnitInHQChain(unr, hq1) &  self.game.HandyFunctionsObj.HowmanyHQsAbove(unr) <  self.game.Data.RuleVar[304] && self.game.Data.UnitObj[unr].IsHQ & self.game.Data.UnitObj[unr].PreDef == -1 & self.game.Data.UnitObj[unr].Regime == self.game.Data.UnitObj[hq1].Regime && self.game.EditObj.TempValue[0].Value[self.game.Data.UnitObj[hq2].X, self.game.Data.UnitObj[hq2].Y] > self.game.EditObj.TempValue[0].Value[self.game.Data.UnitObj[unr].X, self.game.Data.UnitObj[unr].Y])
                  self.game.Data.UnitObj[hq1].HQ = unr;
              }
            }
          }
        }
        let mut unitCounter3: i32 = self.game.Data.UnitCounter;
        for (let mut unr: i32 = 0; unr <= unitCounter3; unr += 1)
        {
          if (self.game.Data.UnitObj[unr].PreDef == -1 && !self.game.Data.UnitObj[unr].IsHQ && self.game.Data.UnitObj[unr].HQ > -1)
          {
            UnitClass[] unitObj = self.game.Data.UnitObj;
            UnitClass[] unitClassArray = unitObj;
            let mut hq: i32 = self.game.Data.UnitObj[unr].HQ;
            let mut index: i32 = hq;
            unitClassArray[index].Supply = unitObj[hq].Supply + self.game.HandyFunctionsObj.UnitSupplyUse(unr) * 2;
          }
        }
      }
      if (self.dosinglestart != 1)
        return;
      let mut mapWidth2: i32 = self.game.Data.MapObj[0].MapWidth;
      for (let mut index1: i32 = 0; index1 <= mapWidth2; index1 += 1)
      {
        let mut mapHeight: i32 = self.game.Data.MapObj[0].MapHeight;
        for (let mut index2: i32 = 0; index2 <= mapHeight; index2 += 1)
        {
          if (self.game.Data.MapObj[0].HexObj[index1, index2].Location > -1)
          {
            if (!self.TownCapitol[self.game.Data.MapObj[0].HexObj[index1, index2].Location])
              self.game.Data.MapObj[0].HexObj[index1, index2].Regime = -1;
          }
          else
            self.game.Data.MapObj[0].HexObj[index1, index2].Regime = -1;
        }
      }
    }

    pub fn OptimizeForAI()
    {
      let mut mapWidth: i32 = self.game.Data.MapObj[0].MapWidth;
      for (let mut index1: i32 = 0; index1 <= mapWidth; index1 += 1)
      {
        let mut mapHeight: i32 = self.game.Data.MapObj[0].MapHeight;
        for (let mut index2: i32 = 0; index2 <= mapHeight; index2 += 1)
        {
          if (!self.game.Data.LandscapeTypeObj[self.game.Data.MapObj[0].HexObj[index1, index2].LandscapeType].IsSea && self.game.Data.MapObj[0].HexObj[index1, index2].Location == -1)
          {
            let mut num1: i32 = 0;
            let mut num2: i32 = index1 - 6;
            let mut num3: i32 = index1 + 6;
            for (let mut x2: i32 = num2; x2 <= num3; x2 += 1)
            {
              let mut num4: i32 = index2 - 6;
              let mut num5: i32 = index2 + 6;
              for (let mut y2: i32 = num4; y2 <= num5; y2 += 1)
              {
                if (x2 >= 0 & y2 >= 0 & x2 <= self.game.Data.MapObj[0].MapWidth & y2 <= self.game.Data.MapObj[0].MapHeight && self.game.Data.MapObj[0].HexObj[x2, y2].Location > -1 && self.game.HandyFunctionsObj.Distance(index1, index2, 0, x2, y2, 0) <= 4 && !self.game.Data.LandscapeTypeObj[self.game.Data.MapObj[0].HexObj[x2, y2].LandscapeType].IsSea)
                  num1 += 1;
              }
            }
            if (num1 == 0)
            {
              self.game.Data.AddLoc(index1, index2, 0);
              self.game.Data.LocObj[self.game.Data.LocCounter].Name = self.GetRandomName(1, -1);
              self.game.Data.LocObj[self.game.Data.LocCounter].Type =  Math.Round( self.game.Data.RuleVar[413]);
              if (self.game.Data.LocTypeObj[ Math.Round( self.game.Data.RuleVar[413])].AutoProd > -1)
              {
                self.game.Data.LocObj[self.game.Data.LocCounter].ProdPercent[0] = 100;
                self.game.Data.LocObj[self.game.Data.LocCounter].Production[0] = self.game.Data.LocTypeObj[ Math.Round( self.game.Data.RuleVar[413])].AutoProd;
              }
              self.game.Data.MapObj[0].HexObj[index1, index2].VP = 1;
              this += 1.totvp;
              self.game.Data.LocObj[self.game.Data.LocCounter].People = 0;
              self.game.Data.LocObj[self.game.Data.LocCounter].StructuralPts = self.game.Data.LocTypeObj[ Math.Round( self.game.Data.RuleVar[413])].StructuralPts;
              self.game.Data.MapObj[0].HexObj[index1, index2].Location = self.game.Data.LocCounter;
              self.game.Data.MapObj[0].HexObj[index1, index2].LandscapeType = self.game.Data.LocTypeObj[ Math.Round( self.game.Data.RuleVar[413])].OverdrawLTNr;
              self.game.Data.MapObj[0].HexObj[index1, index2].SpriteNr = self.game.Data.LocTypeObj[ Math.Round( self.game.Data.RuleVar[413])].OverdrawSpriteNr;
            }
          }
        }
      }
    }

    pub fn PlaceTowns(x: i32, y: i32, let mut overrule: i32 = -1)
    {
      if ( self.game.Data.RuleVar[413] == -1.0 ||  self.game.Data.RuleVar[414] == -1.0 ||  self.game.Data.RuleVar[415] == -1.0 ||  self.game.Data.RuleVar[416] == -1.0)
        return;
      self.tempcount = 0;
      let mut num1: i32 = -1;
      let mut num2: i32 = 100;
      if ( self.game.Data.RuleVar[479] == 1.0)
        num2 =  Math.Round( self.game.Data.RuleVar[479]);
      self.opt8v += self.game.Data.RegimeCounter + 1 - self.opt8v % (self.game.Data.RegimeCounter + 1);
      let mut opt8v: i32 = self.opt8v;
      for (let mut index1: i32 = 1; index1 <= opt8v; index1 += 1)
      {
        let mut num3: i32 = 0;
        while (num3 == 0)
        {
          let mut num4: i32 = 0;
          let mut num5: i32 = 0;
          let mut num6: i32 = 0;
          let mut num7: i32 = 0;
          while (num7 < num2 | num6 <= 2)
          {
            let mut index2: i32 =  Math.Round( (Conversion.Int(VBMath.Rnd() *  (x - 3)) + 2f));
            let mut index3: i32 =  Math.Round( (Conversion.Int(VBMath.Rnd() *  (y - 3)) + 2f));
            let mut num8: i32 = 0;
            let mut tfacing1: i32 = 1;
            Coordinate coordinate;
            do
            {
              coordinate = self.game.HandyFunctionsObj.HexNeighbour(index2, index3, 0, tfacing1);
              if (coordinate.onmap && self.game.HandyFunctionsObj.IsHexHarbourOrSea(coordinate.x, coordinate.y, coordinate.map))
                num8 += 1;
              tfacing1 += 1;
            }
            while (tfacing1 <= 6);
            if (num8 >= 5)
              index2 = -1;
            if ( self.game.Data.RuleVar[481] > 0.0 & index2 > -1)
            {
              if (self.game.Data.MapObj[0].HexObj[index2, index3].AreaCode[ Math.Round( self.game.Data.RuleVar[481])] == 99 &  VBMath.Rnd() < 0.95)
                index2 = -1;
              else if (!(self.Sclimate == 2 | self.Sclimate == 3) & self.game.Data.MapObj[0].HexObj[index2, index3].AreaCode[ Math.Round( self.game.Data.RuleVar[481])] == 1 &  VBMath.Rnd() < 0.8)
                index2 = -1;
              else if (!(self.Sclimate == 2 | self.Sclimate == 3) & self.game.Data.MapObj[0].HexObj[index2, index3].AreaCode[ Math.Round( self.game.Data.RuleVar[481])] == 4 &  VBMath.Rnd() < 0.5)
                index2 = -1;
            }
            if (index2 > -1 && (self.game.Data.MapObj[0].HexObj[index2, index3].LandscapeType == self.GRASS | self.game.Data.MapObj[0].HexObj[index2, index3].LandscapeType == self.LIGHTFOREST) & self.game.Data.MapObj[0].HexObj[index2, index3].Location == -1)
            {
              num7 += 1;
              if (self.tempcount == 0)
              {
                num4 = index2;
                num5 = index3;
                num6 = 99;
              }
              else
              {
                let mut num9: i32 = 99;
                let mut tempcount: i32 = self.tempcount;
                for (let mut index4: i32 = 0; index4 <= tempcount; index4 += 1)
                {
                  let mut num10: i32 = self.game.HandyFunctionsObj.Distance(index2, index3, 0, self.tempx[index4], self.tempy[index4], 0);
                  if (num10 < num9)
                    num9 = num10;
                }
                if (Operators.CompareString(self.domasterfile, "OFFICIAL LADDER/Ladder.ptmaster", false) == 0)
                {
                  let mut tfacing2: i32 = 1;
                  do
                  {
                    coordinate = self.game.HandyFunctionsObj.HexNeighbourSameMap(index2, index3, 0, tfacing2);
                    if (coordinate.onmap && self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].Regime != self.game.Data.MapObj[0].HexObj[index2, index3].Regime && self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].Regime != -1)
                      num9 = 1;
                    tfacing2 += 1;
                  }
                  while (tfacing2 <= 6);
                }
                if (num9 > num6)
                {
                  num6 = num9;
                  num4 = index2;
                  num5 = index3;
                }
              }
            }
          }
          let mut x1: i32 = num4;
          let mut y1: i32 = num5;
          num11: i32;
          if (self.domirror == 1)
          {
            if (num1 == -1)
            {
              let mut num12: i32 =  Math.Round(Conversion.Int( (self.game.Data.MapObj[0].MapWidth + 1) / 2.0));
              let mut num13: i32 =  Math.Round(Conversion.Int( (self.game.Data.MapObj[0].MapHeight + 1) / 2.0));
              let mut num14: i32 = y1 < num13 ? (y1 >= num13 ? num13 : self.game.Data.MapObj[0].MapHeight - y1) : self.game.Data.MapObj[0].MapHeight - y1;
              num1 = x1 < num12 ? (x1 >= num12 ? num12 : self.game.Data.MapObj[0].MapWidth - x1) : self.game.Data.MapObj[0].MapWidth - x1;
              num11 = num14;
            }
            else
            {
              x1 = num1;
              y1 = num11;
              num1 = -1;
              num11 = -1;
            }
          }
          if (self.game.Data.MapObj[0].HexObj[x1, y1].LandscapeType == self.GRASS | self.game.Data.MapObj[0].HexObj[x1, y1].LandscapeType == self.LIGHTFOREST && self.game.Data.MapObj[0].HexObj[x1, y1].Location == -1)
          {
            this += 1.tempcount;
            self.tempx[self.tempcount] = x1;
            self.tempy[self.tempcount] = y1;
            self.game.Data.AddLoc(x1, y1, 0);
            float num15;
            float num16;
            float num17;
            float num18;
            if (self.opt10v == 1)
            {
              num15 = 0.75f;
              num16 = 0.8f;
              num17 = 1f;
              num18 = 2f;
            }
            if (self.opt10v == 2)
            {
              num15 = 0.66f;
              num16 = 0.66f;
              num17 = 2f;
              num18 = 2f;
            }
            if (self.opt10v == 3)
            {
              num15 = 0.25f;
              num16 = 0.5f;
              num17 = 0.5f;
              num18 = 1f;
            }
            if (self.opt10v == 4)
            {
              num15 = 0.0f;
              num16 = 0.4f;
              num17 = 0.4f;
              num18 = 1f;
            }
            float num19 = VBMath.Rnd();
            float num20 = VBMath.Rnd();
            float num21 = VBMath.Rnd();
            float num22 = VBMath.Rnd();
            index5: i32;
            if ( num19 <=  num15)
            {
              index5 =  Math.Round( self.game.Data.RuleVar[413]);
              self.TownSize[self.game.Data.LocCounter] = 0;
              self.game.Data.LocObj[self.game.Data.LocCounter].Name = "";
            }
            else if ( num20 <=  num16)
            {
              self.TownSize[self.game.Data.LocCounter] = 1;
              index5 =  Math.Round( self.game.Data.RuleVar[414]);
              self.game.Data.LocObj[self.game.Data.LocCounter].Name = "";
            }
            else if ( num21 <=  num17)
            {
              self.TownSize[self.game.Data.LocCounter] = 2;
              index5 =  Math.Round( self.game.Data.RuleVar[415]);
              self.game.Data.LocObj[self.game.Data.LocCounter].Name = "";
            }
            else if ( num22 <=  num18)
            {
              self.TownSize[self.game.Data.LocCounter] = 3;
              index5 =  Math.Round( self.game.Data.RuleVar[416]);
              self.game.Data.LocObj[self.game.Data.LocCounter].Name = "";
            }
            if (overrule > -1)
              index5 = overrule;
            num23: i32;
            if (num1 == -1 & self.domirror == 1)
              index5 = num23;
            else
              num23 = index5;
            self.game.Data.LocObj[self.game.Data.LocCounter].Type = index5;
            if (self.game.Data.LocTypeObj[index5].AutoProd > -1)
            {
              self.game.Data.LocObj[self.game.Data.LocCounter].ProdPercent[0] = 100;
              self.game.Data.LocObj[self.game.Data.LocCounter].Production[0] = self.game.Data.LocTypeObj[index5].AutoProd;
            }
            self.game.Data.MapObj[0].HexObj[x1, y1].VP = 1;
            this += 1.totvp;
            self.game.Data.LocObj[self.game.Data.LocCounter].People = 0;
            self.game.Data.LocObj[self.game.Data.LocCounter].StructuralPts = self.game.Data.LocTypeObj[index5].StructuralPts;
            self.game.Data.MapObj[0].HexObj[x1, y1].Location = self.game.Data.LocCounter;
            if (self.game.Data.LocTypeObj[index5].OverdrawLTNr > -1)
            {
              self.game.Data.MapObj[0].HexObj[x1, y1].LandscapeType = self.game.Data.LocTypeObj[index5].OverdrawLTNr;
              self.game.Data.MapObj[0].HexObj[x1, y1].SpriteNr = self.game.Data.LocTypeObj[index5].OverdrawSpriteNr;
            }
            num3 = 1;
          }
        }
      }
    }

    pub fn MakeLakes(x: i32, y: i32)
    {
      let mut num1: i32 = x;
      for (let mut cx: i32 = 0; cx <= num1; cx += 1)
      {
        let mut num2: i32 = y;
        for (let mut cy: i32 = 0; cy <= num2; cy += 1)
        {
          let mut num3: i32 = 0;
          let mut index1: i32 = 0;
          Coordinate coordinate;
          do
          {
            if (self.game.Data.MapObj[0].HexObj[cx, cy].RiverType[index1] > -1)
            {
              num3 += 1;
            }
            else
            {
              coordinate = self.game.HandyFunctionsObj.HexNeighbourSameMap(cx, cy, 0, index1 + 1);
              if (coordinate.onmap && self.game.Data.LandscapeTypeObj[self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].LandscapeType].IsSea)
                num3 += 1;
            }
            index1 += 1;
          }
          while (index1 <= 5);
          if (num3 == 6)
          {
            let mut index2: i32 = 0;
            do
            {
              self.game.Data.MapObj[0].HexObj[cx, cy].RiverType[index2] = -1;
              coordinate = self.game.HandyFunctionsObj.HexNeighbourSameMap(cx, cy, 0, index2 + 1);
              if (coordinate.onmap)
              {
                let mut index3: i32 = index2 + 3;
                if (index3 > 5)
                  index3 -= 6;
                self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].RiverType[index3] = -1;
              }
              index2 += 1;
            }
            while (index2 <= 5);
            self.game.Data.MapObj[0].HexObj[cx, cy].LandscapeType = self.WATER;
          }
        }
      }
    }

    pub GetRandomRegimeName: String(regnr: i32)
    {
      strArray: Vec<String> = new string[1001];
      Random random = Random::new();
      self.Flag1 = "";
      self.Flag1b = "";
      self.RegFavClimate = (int[]) Utils.CopyArray((Array) self.RegFavClimate, (Array) new int[self.game.Data.RegimeCounter + 1]);
      Right: String;
      num1: i32;
      if ( self.game.Data.RuleVar[424] > 0.0)
      {
        num2: i32;
        while (num2 < 100)
        {
          num2 += 1;
          if ( self.game.Data.RuleVar[496] > 0.0 & regnr == self.opt3v)
          {
            let mut randomFromStringList: i32 = self.game.ProcessingObj.GetRandomFromStringList(self.game.HandyFunctionsObj.GetStringListByID( Math.Round( self.game.Data.RuleVar[496])));
            Right = self.game.Data.StringListObj[self.game.HandyFunctionsObj.GetStringListByID( Math.Round( self.game.Data.RuleVar[496]))].Data[randomFromStringList, 1];
            self.Flag1 = self.game.Data.StringListObj[self.game.HandyFunctionsObj.GetStringListByID( Math.Round( self.game.Data.RuleVar[496]))].Data[randomFromStringList, 2];
            self.Flag1b = self.game.Data.StringListObj[self.game.HandyFunctionsObj.GetStringListByID( Math.Round( self.game.Data.RuleVar[496]))].Data[randomFromStringList, 3];
            self.RegFavClimate[regnr] = Conversions.ToInteger(self.game.Data.StringListObj[self.game.HandyFunctionsObj.GetStringListByID( Math.Round( self.game.Data.RuleVar[496]))].Data[randomFromStringList, 4]);
            self.game.Data.RegimeObj[regnr].People =  Math.Round( self.game.Data.RuleVar[497]);
          }
          else if ( self.game.Data.RuleVar[492] > 0.0 & (self.Regid[regnr] + 1 == 7 | self.Regid[regnr] + 1 == 14))
          {
            let mut randomFromStringList: i32 = self.game.ProcessingObj.GetRandomFromStringList(self.game.HandyFunctionsObj.GetStringListByID( Math.Round( self.game.Data.RuleVar[492])));
            Right = self.game.Data.StringListObj[self.game.HandyFunctionsObj.GetStringListByID( Math.Round( self.game.Data.RuleVar[492]))].Data[randomFromStringList, 1];
            self.Flag1 = self.game.Data.StringListObj[self.game.HandyFunctionsObj.GetStringListByID( Math.Round( self.game.Data.RuleVar[492]))].Data[randomFromStringList, 2];
            self.Flag1b = self.game.Data.StringListObj[self.game.HandyFunctionsObj.GetStringListByID( Math.Round( self.game.Data.RuleVar[492]))].Data[randomFromStringList, 3];
            self.RegFavClimate[regnr] = Conversions.ToInteger(self.game.Data.StringListObj[self.game.HandyFunctionsObj.GetStringListByID( Math.Round( self.game.Data.RuleVar[492]))].Data[randomFromStringList, 4]);
          }
          else if ( self.game.Data.RuleVar[488] > 0.0 & (self.Regid[regnr] + 1 == 6 | self.Regid[regnr] + 1 == 13))
          {
            let mut randomFromStringList: i32 = self.game.ProcessingObj.GetRandomFromStringList(self.game.HandyFunctionsObj.GetStringListByID( Math.Round( self.game.Data.RuleVar[488])));
            Right = self.game.Data.StringListObj[self.game.HandyFunctionsObj.GetStringListByID( Math.Round( self.game.Data.RuleVar[488]))].Data[randomFromStringList, 1];
            self.Flag1 = self.game.Data.StringListObj[self.game.HandyFunctionsObj.GetStringListByID( Math.Round( self.game.Data.RuleVar[488]))].Data[randomFromStringList, 2];
            self.Flag1b = self.game.Data.StringListObj[self.game.HandyFunctionsObj.GetStringListByID( Math.Round( self.game.Data.RuleVar[488]))].Data[randomFromStringList, 3];
            self.RegFavClimate[regnr] = Conversions.ToInteger(self.game.Data.StringListObj[self.game.HandyFunctionsObj.GetStringListByID( Math.Round( self.game.Data.RuleVar[488]))].Data[randomFromStringList, 4]);
          }
          else if ( self.game.Data.RuleVar[484] > 0.0 & (self.Regid[regnr] + 1 == 5 | self.Regid[regnr] + 1 == 12))
          {
            let mut randomFromStringList: i32 = self.game.ProcessingObj.GetRandomFromStringList(self.game.HandyFunctionsObj.GetStringListByID( Math.Round( self.game.Data.RuleVar[484])));
            Right = self.game.Data.StringListObj[self.game.HandyFunctionsObj.GetStringListByID( Math.Round( self.game.Data.RuleVar[484]))].Data[randomFromStringList, 1];
            self.Flag1 = self.game.Data.StringListObj[self.game.HandyFunctionsObj.GetStringListByID( Math.Round( self.game.Data.RuleVar[484]))].Data[randomFromStringList, 2];
            self.Flag1b = self.game.Data.StringListObj[self.game.HandyFunctionsObj.GetStringListByID( Math.Round( self.game.Data.RuleVar[484]))].Data[randomFromStringList, 3];
            self.RegFavClimate[regnr] = Conversions.ToInteger(self.game.Data.StringListObj[self.game.HandyFunctionsObj.GetStringListByID( Math.Round( self.game.Data.RuleVar[484]))].Data[randomFromStringList, 4]);
          }
          else if ( self.game.Data.RuleVar[468] > 0.0 & (self.Regid[regnr] + 1 == 4 | self.Regid[regnr] + 1 == 11))
          {
            let mut randomFromStringList: i32 = self.game.ProcessingObj.GetRandomFromStringList(self.game.HandyFunctionsObj.GetStringListByID( Math.Round( self.game.Data.RuleVar[468])));
            Right = self.game.Data.StringListObj[self.game.HandyFunctionsObj.GetStringListByID( Math.Round( self.game.Data.RuleVar[468]))].Data[randomFromStringList, 1];
            self.Flag1 = self.game.Data.StringListObj[self.game.HandyFunctionsObj.GetStringListByID( Math.Round( self.game.Data.RuleVar[468]))].Data[randomFromStringList, 2];
            self.Flag1b = self.game.Data.StringListObj[self.game.HandyFunctionsObj.GetStringListByID( Math.Round( self.game.Data.RuleVar[468]))].Data[randomFromStringList, 3];
            self.RegFavClimate[regnr] = Conversions.ToInteger(self.game.Data.StringListObj[self.game.HandyFunctionsObj.GetStringListByID( Math.Round( self.game.Data.RuleVar[468]))].Data[randomFromStringList, 4]);
          }
          else if ( self.game.Data.RuleVar[434] > 0.0 & (self.Regid[regnr] + 1 == 3 | self.Regid[regnr] + 1 == 10))
          {
            let mut randomFromStringList: i32 = self.game.ProcessingObj.GetRandomFromStringList(self.game.HandyFunctionsObj.GetStringListByID( Math.Round( self.game.Data.RuleVar[434])));
            Right = self.game.Data.StringListObj[self.game.HandyFunctionsObj.GetStringListByID( Math.Round( self.game.Data.RuleVar[434]))].Data[randomFromStringList, 1];
            self.Flag1 = self.game.Data.StringListObj[self.game.HandyFunctionsObj.GetStringListByID( Math.Round( self.game.Data.RuleVar[434]))].Data[randomFromStringList, 2];
            self.Flag1b = self.game.Data.StringListObj[self.game.HandyFunctionsObj.GetStringListByID( Math.Round( self.game.Data.RuleVar[434]))].Data[randomFromStringList, 3];
            self.RegFavClimate[regnr] = Conversions.ToInteger(self.game.Data.StringListObj[self.game.HandyFunctionsObj.GetStringListByID( Math.Round( self.game.Data.RuleVar[434]))].Data[randomFromStringList, 4]);
          }
          else if ( self.game.Data.RuleVar[429] > 0.0 & (self.Regid[regnr] + 1 == 2 | self.Regid[regnr] + 1 == 9))
          {
            let mut randomFromStringList: i32 = self.game.ProcessingObj.GetRandomFromStringList(self.game.HandyFunctionsObj.GetStringListByID( Math.Round( self.game.Data.RuleVar[429])));
            Right = self.game.Data.StringListObj[self.game.HandyFunctionsObj.GetStringListByID( Math.Round( self.game.Data.RuleVar[429]))].Data[randomFromStringList, 1];
            self.Flag1 = self.game.Data.StringListObj[self.game.HandyFunctionsObj.GetStringListByID( Math.Round( self.game.Data.RuleVar[429]))].Data[randomFromStringList, 2];
            self.Flag1b = self.game.Data.StringListObj[self.game.HandyFunctionsObj.GetStringListByID( Math.Round( self.game.Data.RuleVar[429]))].Data[randomFromStringList, 3];
            self.RegFavClimate[regnr] = Conversions.ToInteger(self.game.Data.StringListObj[self.game.HandyFunctionsObj.GetStringListByID( Math.Round( self.game.Data.RuleVar[429]))].Data[randomFromStringList, 4]);
          }
          else if ( self.game.Data.RuleVar[429] > 0.0 & (self.Regid[regnr] + 1 == 1 | self.Regid[regnr] + 1 == 8))
          {
            let mut randomFromStringList: i32 = self.game.ProcessingObj.GetRandomFromStringList(self.game.HandyFunctionsObj.GetStringListByID( Math.Round( self.game.Data.RuleVar[424])));
            Right = self.game.Data.StringListObj[self.game.HandyFunctionsObj.GetStringListByID( Math.Round( self.game.Data.RuleVar[424]))].Data[randomFromStringList, 1];
            self.Flag1 = self.game.Data.StringListObj[self.game.HandyFunctionsObj.GetStringListByID( Math.Round( self.game.Data.RuleVar[424]))].Data[randomFromStringList, 2];
            self.Flag1b = self.game.Data.StringListObj[self.game.HandyFunctionsObj.GetStringListByID( Math.Round( self.game.Data.RuleVar[424]))].Data[randomFromStringList, 3];
            self.RegFavClimate[regnr] = Conversions.ToInteger(self.game.Data.StringListObj[self.game.HandyFunctionsObj.GetStringListByID( Math.Round( self.game.Data.RuleVar[424]))].Data[randomFromStringList, 4]);
          }
          num1 = 0;
          let mut regimeCounter: i32 = self.game.Data.RegimeCounter;
          for (let mut index: i32 = 0; index <= regimeCounter; index += 1)
          {
            if (Operators.CompareString(self.game.Data.RegimeObj[index].Name, Right, false) == 0)
              num1 = 1;
          }
          if (num1 == 0)
            return Right;
        }
      }
      while (num1 == 0)
      {
        strArray[0] = "Fascist";
        strArray[1] = "Dark";
        strArray[2] = "Light";
        strArray[3] = "Holy";
        strArray[4] = "Anamistic";
        strArray[5] = "Democratic";
        strArray[6] = "Hard";
        strArray[7] = "Deep";
        strArray[8] = "Syndicated";
        strArray[9] = "Communist";
        strArray[10] = "Free";
        strArray[11] = "Slave";
        strArray[12] = "Cyborg";
        strArray[13] = "Scientific";
        strArray[14] = "Christian";
        strArray[15] = "Muslim";
        strArray[16] = "Hindu";
        strArray[17] = "Human";
        strArray[18] = "Golden";
        strArray[19] = "Twisted";
        strArray[20] = "Trinity";
        strArray[21] = "Twilight";
        strArray[22] = "Cross";
        strArray[23] = "Shiite";
        strArray[24] = "Suni";
        strArray[25] = "Reformed";
        strArray[26] = "Workers";
        strArray[27] = "Liberal";
        strArray[28] = "Wired";
        strArray[29] = "Outlaw";
        strArray[30] = "Joined";
        strArray[31] = "Old";
        strArray[32] = "Constituted";
        strArray[33] = "Agnostic";
        strArray[34] = "Reformed";
        strArray[35] = "Reformed";
        strArray[36] = "First";
        strArray[37] = "Second";
        strArray[38] = "Third";
        strArray[39] = "Anarchic";
        strArray[40] = "Central";
        str: String = strArray[Conversion.Int(random.Next(0, 40))];
        strArray[0] = " Core";
        strArray[1] = " Alliance";
        strArray[2] = " Force";
        strArray[3] = " Army";
        strArray[4] = " Territory";
        strArray[5] = " Realm";
        strArray[6] = " Federation";
        strArray[7] = " Bond";
        strArray[8] = " Corporation";
        strArray[9] = " Front";
        strArray[10] = " Kingdom";
        strArray[11] = " State";
        strArray[12] = " Country";
        strArray[13] = " Marines";
        strArray[14] = " Killers";
        strArray[15] = " Team";
        strArray[16] = " Union";
        strArray[17] = " Party";
        strArray[18] = " Colony";
        strArray[19] = " Company";
        strArray[20] = " Gang";
        strArray[21] = " Empire";
        strArray[22] = " County";
        strArray[23] = " League";
        strArray[24] = " Org";
        strArray[25] = " Conglomerate";
        strArray[26] = " Autocracy";
        strArray[27] = " Commune";
        strArray[28] = " Brothers";
        strArray[29] = " Church";
        strArray[30] = " Society";
        strArray[31] = " Confederation";
        strArray[32] = " State";
        strArray[33] = " Movement";
        strArray[34] = " Congress";
        strArray[35] = " Imperium";
        strArray[36] = " Chiefdom";
        strArray[37] = " County";
        strArray[38] = " Barony";
        strArray[39] = " Settlement";
        strArray[40] = " Oligarchy";
        Right = str + strArray[Conversion.Int(random.Next(0, 40))];
        num1 = 1;
        let mut namecount: i32 = self.namecount;
        for (let mut index: i32 = 0; index <= namecount; index += 1)
        {
          if (Operators.CompareString(self.namelist[index], Right, false) == 0)
          {
            num1 = 0;
            break;
          }
        }
      }
      this += 1.namecount;
      self.namelist[self.namecount] = Right;
      return Right;
    }

    pub GetRandomName: String(townsize: i32, townppl: i32, bool IsCapitol = false)
    {
      strArray: Vec<String> = new string[10000];
      Random random = Random::new();
      Right: String;
      if ( self.game.Data.RuleVar[440 + townsize] > 0.0)
      {
        num1: i32;
        while (num1 < 1000)
        {
          num1 += 1;
          let mut index1: i32 = 440 + townsize;
          if (IsCapitol &  self.game.Data.RuleVar[500] > 0.0)
            index1 = 500;
          let mut index2: i32 = self.game.ProcessingObj.GetRandomFromStringList(self.game.HandyFunctionsObj.GetStringListByID( Math.Round( self.game.Data.RuleVar[index1])));
          Right = self.game.Data.StringListObj[self.game.HandyFunctionsObj.GetStringListByID( Math.Round( self.game.Data.RuleVar[index1]))].Data[index2, 1];
          if (townppl > -1 &&  townppl != Conversion.Val(Strings.Trim(self.game.Data.StringListObj[self.game.HandyFunctionsObj.GetStringListByID( Math.Round( self.game.Data.RuleVar[index1]))].Data[index2, 2])))
            index2 = -1;
          if (index2 > -1)
          {
            let mut num2: i32 = 0;
            let mut locCounter: i32 = self.game.Data.LocCounter;
            for (let mut index3: i32 = 0; index3 <= locCounter; index3 += 1)
            {
              if (Operators.CompareString(self.game.Data.LocObj[index3].Name, Right, false) == 0)
                num2 = 1;
            }
            if (num2 == 0)
              return Right;
          }
        }
      }
      let mut num3: i32 = 0;
      while (num3 == 0)
      {
        Right = "";
        let mut num4: i32 = 0;
        index4: i32;
        if (DrawMod.RandyNumber.Next(0, 100) < 20)
        {
          strArray[0] = "High ";
          strArray[1] = "Low ";
          strArray[2] = "First ";
          strArray[3] = "New ";
          strArray[4] = "Old ";
          strArray[5] = "Far ";
          strArray[6] = "Top ";
          strArray[7] = "North ";
          strArray[8] = "Lake ";
          strArray[9] = "Camp ";
          strArray[10] = "Fort ";
          strArray[11] = "Keep ";
          strArray[12] = "Base ";
          strArray[13] = "Central ";
          strArray[14] = "Inner ";
          strArray[15] = "Outer ";
          strArray[16] = "Major ";
          strArray[17] = "Minor ";
          strArray[18] = "Second";
          strArray[19] = "Our";
          strArray[20] = "Purple";
          strArray[21] = "Yellow";
          strArray[22] = "Blue";
          strArray[23] = "Mount";
          strArray[24] = "Tar";
          strArray[25] = "Tin";
          num4 = 1;
          index4 += Conversion.Int(DrawMod.RandyNumber.Next(0, 25));
          Right += strArray[index4];
        }
        let mut num5: i32 = 0;
        if (DrawMod.RandyNumber.Next(0, 100) < 50)
        {
          strArray[0] = "Glory";
          strArray[1] = "Timber";
          strArray[2] = "Arion";
          strArray[3] = "Beaver";
          strArray[4] = "Boat";
          strArray[5] = "Choron";
          strArray[6] = "Deflec";
          strArray[7] = "Ennix";
          strArray[8] = "Fairsight";
          strArray[9] = "Gargoyle";
          strArray[10] = "Heaven";
          strArray[11] = "Orion";
          strArray[12] = "Palmtree";
          strArray[13] = "Flower";
          strArray[14] = "Cowsy";
          strArray[15] = "Deserdo";
          strArray[16] = "Frenchy";
          strArray[17] = "Hardskin";
          strArray[18] = "Ranger";
          strArray[19] = "Deepmist";
          strArray[20] = "Bluelight";
          strArray[21] = "Reddawn";
          strArray[22] = "Greenberry";
          strArray[23] = "Goldmine";
          strArray[24] = "Silver";
          strArray[25] = "Italian";
          strArray[26] = "Fachatto";
          strArray[27] = "Rainback";
          strArray[28] = "Heartlong";
          strArray[29] = "Schuman";
          strArray[30] = "Karl Marx";
          strArray[31] = "Bushbane";
          strArray[32] = "Darkon";
          strArray[33] = "Elfas";
          strArray[34] = "Voroth";
          strArray[35] = "Kinsman";
          strArray[36] = "Lightsky";
          strArray[37] = "Mozart";
          strArray[38] = "Balbo";
          strArray[39] = "Kinskey";
          strArray[40] = "Fallow";
          strArray[41] = "Hollow";
          strArray[42] = "Beech";
          strArray[43] = "Rotown";
          strArray[44] = "Weather";
          strArray[45] = "Valley";
          strArray[46] = "Triad";
          strArray[47] = "Dragon";
          strArray[48] = "Unicorn";
          strArray[49] = "Buffalo";
          strArray[50] = "Morning";
          strArray[51] = "Granite";
          strArray[52] = "Potato";
          strArray[53] = "Quicksilver";
          strArray[54] = "Palmas";
          strArray[55] = "Sandbeach";
          strArray[56] = "Crystaline";
          strArray[57] = "Nighttown";
          strArray[58] = "Darking";
          strArray[59] = "Madding";
          strArray[60] = "Paramac";
          strArray[61] = "Twinning";
          strArray[62] = "Bellow";
          strArray[63] = "Groundhawk";
          strArray[64] = "Pinetree";
          strArray[65] = "Albatross";
          strArray[66] = "Kirkee";
          strArray[67] = "Pottam";
          strArray[68] = "Hotten";
          strArray[69] = "Fard";
          strArray[70] = "Elephant";
          strArray[71] = "Minister";
          strArray[72] = "Parliament";
          strArray[73] = "Satin";
          strArray[74] = "Platen";
          strArray[75] = "Eagle";
          strArray[76] = "Watcher";
          strArray[77] = "Caven";
          strArray[78] = "Mansion";
          strArray[79] = "Rock";
          strArray[80] = "Frontier";
          Right += strArray[Conversion.Int(DrawMod.RandyNumber.Next(0, 80))];
          num5 = 1;
        }
        if (!((DrawMod.RandyNumber.Next(0, 100) < 60 | num5 == 1) & num5 != 0 | num4 == 1))
        {
          strArray[0] = "Deep";
          strArray[1] = "Tree";
          strArray[2] = "Sun";
          strArray[3] = "Wax";
          strArray[4] = "Lilly";
          strArray[5] = "Blue";
          strArray[6] = "Aard";
          strArray[7] = "Bee";
          strArray[8] = "Stone";
          strArray[9] = "Glass";
          strArray[10] = "Down";
          strArray[11] = "Fin";
          strArray[12] = "Weed";
          strArray[13] = "Queens";
          strArray[14] = "Kings";
          strArray[15] = "Love";
          strArray[16] = "Mins";
          strArray[17] = "Tir";
          strArray[18] = "Horse";
          strArray[19] = "Dip";
          strArray[20] = "Water";
          strArray[21] = "Doop";
          strArray[22] = "Bush";
          strArray[23] = "Moon";
          strArray[24] = "Stiff";
          strArray[25] = "Rose";
          strArray[26] = "Green";
          strArray[27] = "Glor";
          strArray[28] = "Krin";
          strArray[29] = "Dirt";
          strArray[30] = "Metal";
          strArray[31] = "Up";
          strArray[32] = "Mirk";
          strArray[33] = "Cult";
          strArray[34] = "Lords";
          strArray[35] = "Pearl";
          strArray[36] = "Hate";
          strArray[37] = "Por";
          strArray[38] = "Zul";
          strArray[39] = "Cattle";
          strArray[40] = "Tin";
          strArray[41] = "Land";
          let mut index5: i32 = Conversion.Int(DrawMod.RandyNumber.Next(0, 40));
          str: String = Right + strArray[index5];
          strArray[0] = "castle";
          strArray[1] = "house";
          strArray[2] = "work";
          strArray[3] = "dam";
          strArray[4] = "ton";
          strArray[5] = "hold";
          strArray[6] = "keep";
          strArray[7] = "pin";
          strArray[8] = "lor";
          strArray[9] = "aleen";
          strArray[10] = "fish";
          strArray[11] = "road";
          strArray[12] = "square";
          strArray[13] = "sight";
          strArray[14] = "fresh";
          strArray[15] = "brick";
          strArray[16] = "keen";
          strArray[17] = "ly";
          strArray[18] = "rod";
          strArray[19] = "desert";
          strArray[20] = "forest";
          strArray[21] = "wall";
          strArray[22] = "market";
          strArray[23] = "sleep";
          strArray[24] = "flow";
          strArray[25] = "mok";
          strArray[26] = "fall";
          strArray[27] = "stuff";
          strArray[28] = "pouch";
          strArray[29] = "varn";
          strArray[30] = "elath";
          strArray[31] = "beatle";
          strArray[32] = "path";
          strArray[33] = "center";
          strArray[34] = "cellar";
          strArray[35] = "salt";
          strArray[36] = "timber";
          strArray[37] = "ion";
          strArray[38] = "sky";
          strArray[39] = "staff";
          strArray[40] = "meadows";
          strArray[41] = "wood";
          index4 = Conversion.Int(DrawMod.RandyNumber.Next(0, 40));
          Right = str + strArray[index4];
        }
        if ( VBMath.Rnd() < 0.3)
        {
          strArray[0] = " Ville";
          strArray[1] = " Town";
          strArray[2] = " Hope";
          strArray[3] = "crossing";
          strArray[4] = "cross";
          strArray[5] = " Soul";
          strArray[6] = "hill";
          strArray[7] = "field";
          strArray[8] = "ford";
          strArray[9] = "shire";
          strArray[10] = " End";
          strArray[11] = " Denn";
          strArray[12] = " Fir";
          strArray[13] = " Varn";
          strArray[14] = "ing";
          strArray[15] = "gate";
          strArray[16] = " View";
          strArray[17] = " Hole";
          strArray[18] = "stream";
          strArray[19] = "s View";
          strArray[20] = " Dream";
          index4 = Conversion.Int(DrawMod.RandyNumber.Next(0, 20));
          Right += strArray[index4];
        }
        num3 = 1;
        let mut namecount: i32 = self.namecount;
        for (let mut index6: i32 = 0; index6 <= namecount; index6 += 1)
        {
          if (Operators.CompareString(self.namelist[index6], Right, false) == 0)
          {
            num3 = 0;
            break;
          }
        }
      }
      this += 1.namecount;
      self.namelist[self.namecount] = Right;
      return Right;
    }

    pub fn DrawARiverAddRiver(x: i32, y: i32, z: i32, steppy: i32, ox: i32, oy: i32, oz: i32)
    {
      object[,] objArray = new object[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      if (self.game.Data.MapObj[0].HexObj[x, y].RiverType[z] == -1)
        self.game.Data.MapObj[0].HexObj[x, y].RiverType[z] = self.SMALLRIVER;
      else
        self.game.Data.MapObj[0].HexObj[x, y].RiverType[z] = self.BIGRIVER;
      Coordinate coordinate = self.game.HandyFunctionsObj.HexNeighbour(x, y, 0, z + 1);
      if (coordinate.onmap)
      {
        let mut index: i32 = z + 3;
        if (index > 5)
          index -= 6;
        self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].RiverType[index] = self.game.Data.MapObj[0].HexObj[x, y].RiverType[z];
        if (steppy > -1)
          self.curriv[coordinate.x, coordinate.y, index] = 1;
        if (steppy > -1)
          self.rivstep[coordinate.x, coordinate.y, index] = steppy;
      }
      if (steppy > -1)
        self.curriv[x, y, z] = 1;
      if (steppy > -1)
        self.rivstep[x, y, z] = steppy;
      if (ox <= -1)
        return;
      self.nextrivstep[ox, oy, oz].x = x;
      self.nextrivstep[ox, oy, oz].y = y;
      self.nextrivstep[ox, oy, oz].data1 = z;
      coordinate = self.game.HandyFunctionsObj.HexNeighbour(ox, oy, 0, oz + 1);
      if (!coordinate.onmap)
        return;
      let mut index1: i32 = oz + 3;
      if (index1 > 5)
        index1 -= 6;
      self.nextrivstep[coordinate.x, coordinate.y, index1].x = x;
      self.nextrivstep[coordinate.x, coordinate.y, index1].y = y;
      self.nextrivstep[coordinate.x, coordinate.y, index1].data1 = z;
    }

    pub fn TraceRiver(x: i32, y: i32, z: i32, ox: i32, oy: i32, oz: i32)
    {
      numArray: Vec<i32> = new int[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      Coordinate coordinate;
      coordinate.onmap = false;
      let mut num1: i32 = 1;
      let mut num2: i32 = 0;
      while (num1 == 1 & num2 < 200)
      {
        num1 = 0;
        num2 += 1;
        self.DrawARiverAddRiver(x, y, z, -1, ox, oy, oz);
        if (self.nextrivstep[x, y, z].x > -1)
        {
          num1 = 1;
          ox = x;
          oy = y;
          oz = z;
          x = self.nextrivstep[ox, oy, oz].x;
          y = self.nextrivstep[ox, oy, oz].y;
          z = self.nextrivstep[ox, oy, oz].data1;
        }
      }
    }

    pub fn DrawARiver2(x: i32, y: i32, z: i32)
    {
      numArray: Vec<i32> = new int[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      self.curriv = new int[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1, 6];
      let mut index1: i32 = z + 1;
      if (index1 > 5)
        index1 = 0;
      if (self.game.Data.MapObj[0].HexObj[x, y].RiverType[index1] > -1)
        return;
      let mut index2: i32 = z - 1;
      if (index2 < 0)
        index2 = 5;
      if (self.game.Data.MapObj[0].HexObj[x, y].RiverType[index2] > -1)
        return;
      Coordinate coordinate1 = self.game.HandyFunctionsObj.HexNeighbour(x, y, 0, z);
      if (coordinate1.onmap)
      {
        let mut num: i32 = z + 3;
        if (num > 5)
          num -= 6;
        let mut index3: i32 = num + 1;
        if (index3 > 5)
          index3 = 0;
        if (self.game.Data.MapObj[0].HexObj[coordinate1.x, coordinate1.y].RiverType[index3] > -1)
          return;
        let mut index4: i32 = num - 1;
        if (index4 < 0)
          index4 = 5;
        if (self.game.Data.MapObj[0].HexObj[coordinate1.x, coordinate1.y].RiverType[index4] > -1)
          return;
      }
      let mut num1: i32 = 1;
      if (self.game.Data.MapObj[0].HexObj[x, y].RiverType[z] == self.BIGRIVER)
        return;
      self.DrawARiverAddRiver(x, y, z, 0, -1, -1, -1);
      numArray[x, y] = 1;
      coordinate1 = self.game.HandyFunctionsObj.HexNeighbour(x, y, 0, z + 1);
      if (coordinate1.onmap)
        numArray[coordinate1.x, coordinate1.y] = 1;
      steppy: i32;
      while (self.game.EditObj.TempValue[0].Value[x, y] > 0 & steppy < 200)
      {
        num1 = 0;
        steppy += 1;
        numArray[x, y] = 1;
        let mut num2: i32 = 999999;
        Coordinate coordinate2;
        coordinate2.onmap = false;
        let mut num3: i32 = 0;
        num4: i32;
        do
        {
          coordinate1 = self.game.HandyFunctionsObj.HexNeighbour(x, y, 0, num3 + 1);
          if (coordinate1.onmap & num3 != z && self.game.EditObj.TempValue[0].Value[coordinate1.x, coordinate1.y] < num2 && numArray[coordinate1.x, coordinate1.y] < 1)
          {
            num2 = self.game.EditObj.TempValue[0].Value[coordinate1.x, coordinate1.y];
            coordinate2.x = coordinate1.x;
            coordinate2.y = coordinate1.y;
            coordinate2.onmap = coordinate1.onmap;
            num4 = num3;
          }
          num3 += 1;
        }
        while (num3 <= 5);
        if (!coordinate2.onmap)
          break;
        let mut num5: i32 = 0;
        if (z == 0 & num4 <= 2)
          num5 = 1;
        if (z == 0 & num4 > 3)
          num5 = 2;
        if (z == 1 & num4 <= 3)
          num5 = 1;
        if (z == 2 & num4 <= 4)
          num5 = 1;
        if (z == 3 & num4 <= 5)
          num5 = 1;
        if (z == 4 & (num4 == 5 | num4 == 0))
          num5 = 1;
        if (z == 5 & (num4 == 0 | num4 == 1))
          num5 = 1;
        if (z == 1 & (num4 == 5 | num4 == 0))
          num5 = 2;
        if (z == 2 & num4 <= 1)
          num5 = 2;
        if (z == 3 & (num4 == 1 | num4 == 2))
          num5 = 2;
        if (z == 4 & (num4 == 2 | num4 == 3))
          num5 = 2;
        if (z == 5 & (num4 == 3 | num4 == 4))
          num5 = 2;
        if (num5 == 0)
          num5 = (x + y) % 2 != 0 ? 2 : 1;
        let mut index5: i32 = z;
        if (num5 == 1)
          index5 += 1;
        if (num5 == 2)
          --index5;
        if (index5 > 5)
          index5 -= 6;
        if (0 > index5)
          index5 += 6;
        if (self.game.Data.MapObj[0].HexObj[x, y].RiverType[index5] > -1 & self.curriv[x, y, index5] > 0)
          num5 = num5 != 1 ? 1 : 2;
        let mut num6: i32 = 0;
        while (num6 == 0)
        {
          let mut ox1: i32 = x;
          let mut oy1: i32 = y;
          let mut oz1: i32 = z;
          if (num5 == 1)
            z += 1;
          if (num5 == 2)
            --z;
          if (z > 5)
            z -= 6;
          if (0 > z)
            z += 6;
          coordinate1 = self.game.HandyFunctionsObj.HexNeighbour(x, y, 0, z + 1);
          if (!coordinate1.onmap)
            return;
          if (self.game.Data.MapObj[0].HexObj[x, y].RiverType[z] > -1 & self.curriv[x, y, z] == 0)
          {
            self.TraceRiver(x, y, z, ox1, oy1, oz1);
            return;
          }
          self.DrawARiverAddRiver(x, y, z, steppy, ox1, oy1, oz1);
          numArray[coordinate1.x, coordinate1.y] = 1;
          if (coordinate1.x == coordinate2.x & coordinate1.y == coordinate2.y)
          {
            let mut ox2: i32 = x;
            let mut oy2: i32 = y;
            let mut oz2: i32 = z;
            z += 3;
            if (z > 5)
              z -= 6;
            if (num5 == 1)
              --z;
            if (num5 == 2)
              z += 1;
            if (z > 5)
              z -= 6;
            if (0 > z)
              z += 6;
            x = coordinate2.x;
            y = coordinate2.y;
            if (self.game.Data.MapObj[0].HexObj[x, y].RiverType[z] > -1 & self.curriv[x, y, z] == 0)
            {
              self.TraceRiver(x, y, z, ox2, oy2, oz2);
              return;
            }
            self.DrawARiverAddRiver(x, y, z, steppy, ox2, oy2, oz2);
            coordinate1 = self.game.HandyFunctionsObj.HexNeighbour(x, y, 0, z + 1);
            if (coordinate1.onmap)
              numArray[coordinate1.x, coordinate1.y] = 1;
            num6 = 1;
          }
        }
      }
    }

    pub fn DrawARiver(x: i32, y: i32, z: i32)
    {
      object[,,] objArray = new object[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1, 6];
      float num1 = VBMath.Rnd();
      let mut num2: i32 = 0;
      do
      {
        num2 += 1;
        if (self.game.Data.MapObj[0].HexObj[x, y].RiverType[z] == -1)
        {
          if ( num1 < 0.6)
            self.game.Data.MapObj[0].HexObj[x, y].RiverType[z] = self.SMALLRIVER;
          else
            self.game.Data.MapObj[0].HexObj[x, y].RiverType[z] = self.BIGRIVER;
          objArray[x, y, z] =  1;
        }
        else
        {
          self.game.Data.MapObj[0].HexObj[x, y].RiverType[z] = self.BIGRIVER;
          objArray[x, y, z] =  1;
        }
        if (self.game.EditObj.TempValue[0].Value[x, y] > 1)
        {
          numArray1: Vec<i32> = self.game.EditObj.TempValue[0].Value;
          numArray2: Vec<i32> = numArray1;
          let mut index1: i32 = x;
          let mut index2: i32 = index1;
          let mut index3: i32 = y;
          let mut index4: i32 = index3;
          let mut num3: i32 = numArray1[index1, index3] - 1;
          numArray2[index2, index4] = num3;
        }
        Coordinate coordinate1 = self.game.HandyFunctionsObj.HexNeighbourSameMap(x, y, 0, z + 1);
        let mut index5: i32 = z + 3;
        if (index5 > 5)
          index5 -= 6;
        if (coordinate1.onmap)
        {
          if (self.game.EditObj.TempValue[0].Value[coordinate1.x, coordinate1.y] > 1)
          {
            numArray3: Vec<i32> = self.game.EditObj.TempValue[0].Value;
            numArray4: Vec<i32> = numArray3;
            let mut x1: i32 = coordinate1.x;
            let mut index6: i32 = x1;
            let mut y1: i32 = coordinate1.y;
            let mut index7: i32 = y1;
            let mut num4: i32 = numArray3[x1, y1] - 1;
            numArray4[index6, index7] = num4;
          }
          if (self.game.Data.MapObj[0].HexObj[coordinate1.x, coordinate1.y].RiverType[index5] == -1)
          {
            if ( num1 < 0.6)
              self.game.Data.MapObj[0].HexObj[coordinate1.x, coordinate1.y].RiverType[index5] = self.SMALLRIVER;
            else
              self.game.Data.MapObj[0].HexObj[coordinate1.x, coordinate1.y].RiverType[index5] = self.BIGRIVER;
            objArray[coordinate1.x, coordinate1.y, index5] =  1;
          }
          else
          {
            self.game.Data.MapObj[0].HexObj[coordinate1.x, coordinate1.y].RiverType[index5] = self.BIGRIVER;
            objArray[coordinate1.x, coordinate1.y, index5] =  1;
          }
        }
        let mut num5: i32 = self.game.EditObj.TempValue[0].Value[x, y];
        let mut num6: i32 = z - 1;
        if (0 > num6)
          num6 = 5;
        coordinate1 = self.game.HandyFunctionsObj.HexNeighbourSameMap(x, y, 0, num6 + 1);
        let mut num7: i32 = 999999;
        if (coordinate1.onmap)
          num7 = self.game.EditObj.TempValue[0].Value[coordinate1.x, coordinate1.y];
        let mut num8: i32 = z;
        coordinate1 = self.game.HandyFunctionsObj.HexNeighbourSameMap(x, y, 0, num8 + 1);
        let mut num9: i32 = 999999;
        if (coordinate1.onmap)
          num9 = self.game.EditObj.TempValue[0].Value[coordinate1.x, coordinate1.y];
        let mut num10: i32 = z + 1;
        if (num10 > 5)
          num10 = 0;
        coordinate1 = self.game.HandyFunctionsObj.HexNeighbourSameMap(x, y, 0, num10 + 1);
        let mut num11: i32 = 999999;
        if (coordinate1.onmap)
          num11 = self.game.EditObj.TempValue[0].Value[coordinate1.x, coordinate1.y];
        let mut num12: i32 = 0;
        let mut num13: i32 = 0;
        let mut num14: i32 = 0;
        let mut num15: i32 = 0;
        let mut index8: i32 = z - 1;
        if (0 > index8)
          index8 = 5;
        if (Operators.ConditionalCompareObjectEqual(objArray[x, y, index8],  1, false))
          num12 = 200000;
        coordinate1 = self.game.HandyFunctionsObj.HexNeighbourSameMap(x, y, 0, z + 1);
        if (coordinate1.onmap)
        {
          let mut num16: i32 = z + 3;
          if (num16 > 5)
            num16 -= 6;
          let mut index9: i32 = num16 + 1;
          if (index9 > 5)
            index9 = 0;
          if (Operators.ConditionalCompareObjectEqual(objArray[coordinate1.x, coordinate1.y, index9],  1, false))
            num13 = 200000;
          let mut num17: i32 = z + 3;
          if (num17 > 5)
            num17 -= 6;
          let mut index10: i32 = num17 - 1;
          if (index10 < 0)
            index10 = 5;
          if (Operators.ConditionalCompareObjectEqual(objArray[coordinate1.x, coordinate1.y, index10],  1, false))
            num14 = 200000;
        }
        let mut index11: i32 = z + 1;
        if (5 < index11)
          index11 = 0;
        if (Operators.ConditionalCompareObjectEqual(objArray[x, y, index11],  1, false))
          num15 = 200000;
        let mut num18: i32 = 0;
        let mut num19: i32 = 999999;
        if (num5 + num7 + num12 < num19)
        {
          num19 = num5 + num7 + num12;
          num18 = 1;
        }
        if (num7 + num9 + num13 < num19)
        {
          num19 = num7 + num9 + num13;
          num18 = 2;
        }
        if (num9 + num11 + num14 < num19)
        {
          num19 = num9 + num11 + num14;
          num18 = 3;
        }
        if (num5 + num11 + num15 < num19)
        {
          let mut num20: i32 = num5 + num11 + num15;
          num18 = 4;
        }
        if (num5 == 0 | num7 == 0 | num9 == 0 | num11 == 0 || num5 >= 999999 | num7 >= 999999 | num9 >= 999999 | num11 >= 999999)
          return;
        Coordinate coordinate2;
        if (num18 == 1)
        {
          coordinate1 = self.game.HandyFunctionsObj.HexNeighbourSameMap(x, y, 0, z + 1);
          let mut num21: i32 = z + 3;
          if (num21 > 5)
            num21 -= 6;
          let mut index12: i32 = num21 + 1;
          if (index12 > 5)
            index12 = 0;
          objArray[coordinate1.x, coordinate1.y, index12] =  1;
          coordinate2 = self.game.HandyFunctionsObj.HexNeighbourSameMap(coordinate1.x, coordinate1.y, 0, index12 + 1);
          let mut index13: i32 = index12 + 3;
          if (index13 > 5)
            index13 -= 6;
          objArray[coordinate2.x, coordinate2.y, index13] =  1;
        }
        if (num18 == 1)
        {
          --z;
          if (0 > z)
            z = 5;
          x = x;
          y = y;
        }
        if (num18 == 2)
        {
          let mut index14: i32 = z - 1;
          if (0 > index14)
            index14 = 5;
          objArray[x, y, index14] =  1;
          coordinate2 = self.game.HandyFunctionsObj.HexNeighbourSameMap(x, y, 0, index14 + 1);
          let mut index15: i32 = index14 + 3;
          if (index15 > 5)
            index15 -= 6;
          objArray[coordinate2.x, coordinate2.y, index15] =  1;
        }
        if (num18 == 2)
        {
          coordinate1 = self.game.HandyFunctionsObj.HexNeighbourSameMap(x, y, 0, z + 1);
          let mut num22: i32 = z + 3;
          if (num22 > 5)
            num22 -= 6;
          x = coordinate1.x;
          y = coordinate1.y;
          z = num22;
          z += 1;
          if (z > 5)
            z = 0;
        }
        if (num18 == 3)
        {
          let mut index16: i32 = z + 1;
          if (index16 > 5)
            index16 = 0;
          objArray[x, y, index16] =  1;
          coordinate2 = self.game.HandyFunctionsObj.HexNeighbourSameMap(x, y, 0, index16 + 1);
          let mut index17: i32 = index16 + 3;
          if (index17 > 5)
            index17 -= 6;
          objArray[coordinate2.x, coordinate2.y, index17] =  1;
        }
        if (num18 == 3)
        {
          coordinate1 = self.game.HandyFunctionsObj.HexNeighbourSameMap(x, y, 0, z + 1);
          let mut num23: i32 = z + 3;
          if (num23 > 5)
            num23 -= 6;
          x = coordinate1.x;
          y = coordinate1.y;
          z = num23;
          --z;
          if (z < 0)
            z = 5;
        }
        if (num18 == 4)
        {
          coordinate1 = self.game.HandyFunctionsObj.HexNeighbourSameMap(x, y, 0, z + 1);
          let mut num24: i32 = z + 3;
          if (num24 > 5)
            num24 -= 6;
          let mut index18: i32 = num24 - 1;
          if (index18 < 0)
            index18 = 5;
          objArray[coordinate1.x, coordinate1.y, index18] =  1;
          coordinate2 = self.game.HandyFunctionsObj.HexNeighbourSameMap(coordinate1.x, coordinate1.y, 0, index18 + 1);
          let mut index19: i32 = index18 + 3;
          if (index19 > 5)
            index19 -= 6;
          objArray[coordinate2.x, coordinate2.y, index19] =  1;
        }
        if (num18 == 4)
        {
          z += 1;
          if (z > 5)
            z = 0;
          x = x;
          y = y;
        }
      }
      while (num2 < 75);
      self.game.Data.MapObj[0].HexObj[x, y].LandscapeType = self.WATER;
      self.game.EditObj.TempValue[0].Value[x, y] = 0;
    }

    pub fn MakeHeightTable()
    {
      self.game.EditObj.TempValue = new MapMatrix2[1];
      self.game.EditObj.TempValue2 = new MapMatrix2[1];
      self.game.EditObj.TempValue[0] = new MapMatrix2(self.game.Data.MapObj[0].MapWidth, self.game.Data.MapObj[0].MapHeight);
      self.game.EditObj.TempValue2[0] = new MapMatrix2(self.game.Data.MapObj[0].MapWidth, self.game.Data.MapObj[0].MapHeight);
      let mut mapWidth1: i32 = self.game.Data.MapObj[0].MapWidth;
      for (let mut index1: i32 = 0; index1 <= mapWidth1; index1 += 1)
      {
        let mut mapHeight: i32 = self.game.Data.MapObj[0].MapHeight;
        for (let mut index2: i32 = 0; index2 <= mapHeight; index2 += 1)
        {
          self.game.EditObj.TempValue[0].Value[index1, index2] = 0;
          if (self.game.Data.MapObj[0].HexObj[index1, index2].LandscapeType == self.LOWMOUNTAIN)
            self.game.EditObj.TempValue[0].Value[index1, index2] = 20000;
          else if (self.game.Data.MapObj[0].HexObj[index1, index2].LandscapeType == self.HIGHMOUNTAIN)
            self.game.EditObj.TempValue[0].Value[index1, index2] = 50000;
          else if (self.game.Data.MapObj[0].HexObj[index1, index2].LandscapeType == self.WATER)
            self.game.EditObj.TempValue[0].Value[index1, index2] = 0;
          else if ( VBMath.Rnd() < 0.99 | self.opt4v < 100 | self.game.Data.MapObj[0].HexObj[index1, index2].Regime > -1 | self.game.Data.MapObj[0].HexObj[index1, index2].Location > -1)
          {
            self.game.EditObj.TempValue[0].Value[index1, index2] =  Math.Round( (8000f + Conversion.Int(VBMath.Rnd() * 10000f)));
          }
          else
          {
            self.game.EditObj.TempValue[0].Value[index1, index2] = 0;
            self.game.Data.MapObj[0].HexObj[index1, index2].LandscapeType = self.WATER;
          }
          if ( self.game.Data.RuleVar[481] > 0.0 && self.game.Data.MapObj[0].HexObj[index1, index2].AreaCode[ Math.Round( self.game.Data.RuleVar[481])] == 99)
          {
            numArray1: Vec<i32> = self.game.EditObj.TempValue[0].Value;
            numArray2: Vec<i32> = numArray1;
            let mut index3: i32 = index1;
            let mut index4: i32 = index3;
            let mut index5: i32 = index2;
            let mut index6: i32 = index5;
            let mut num1: i32 = numArray1[index3, index5] * 2;
            numArray2[index4, index6] = num1;
            numArray3: Vec<i32> = self.game.EditObj.TempValue[0].Value;
            numArray4: Vec<i32> = numArray3;
            let mut index7: i32 = index1;
            let mut index8: i32 = index7;
            let mut index9: i32 = index2;
            let mut index10: i32 = index9;
            let mut num2: i32 = numArray3[index7, index9] + 5000;
            numArray4[index8, index10] = num2;
          }
          if (self.domirror == 1)
          {
            let mut num: i32 =  Math.Round( self.game.Data.MapObj[0].MapWidth / 2.0);
            if (index1 >= num - 1 & index1 <= num + 1)
              self.game.EditObj.TempValue[0].Value[index1, index2] = 20000;
          }
          if (index1 == 0 | index2 == 0)
            self.game.EditObj.TempValue[0].Value[index1, index2] = 0;
          if (index1 == self.game.Data.MapObj[0].MapWidth | index2 == self.game.Data.MapObj[0].MapHeight)
            self.game.EditObj.TempValue[0].Value[index1, index2] = 0;
        }
      }
      let mut num3: i32 = 0;
      Coordinate coordinate;
      do
      {
        num3 += 1;
        let mut mapWidth2: i32 = self.game.Data.MapObj[0].MapWidth;
        for (let mut cx: i32 = 0; cx <= mapWidth2; cx += 1)
        {
          let mut mapHeight: i32 = self.game.Data.MapObj[0].MapHeight;
          for (let mut cy: i32 = 0; cy <= mapHeight; cy += 1)
          {
            if (self.game.Data.MapObj[0].HexObj[cx, cy].LandscapeType != self.WATER)
            {
              let mut num4: i32 = 0;
              let mut num5: i32 = 0;
              let mut tfacing: i32 = 1;
              do
              {
                coordinate = self.game.HandyFunctionsObj.HexNeighbourSameMap(cx, cy, 0, tfacing);
                if (coordinate.onmap)
                {
                  num5 += 1;
                  num4 += self.game.EditObj.TempValue[0].Value[coordinate.x, coordinate.y];
                }
                tfacing += 1;
              }
              while (tfacing <= 6);
              if (num5 > 0 & num4 > num5)
              {
                let mut num6: i32 =  Math.Round(Conversion.Int( num4 /  num5));
                if (num6 < 1)
                  num6 = 1;
                self.game.EditObj.TempValue2[0].Value[cx, cy] = num6;
              }
              else
                self.game.EditObj.TempValue2[0].Value[cx, cy] = self.game.EditObj.TempValue[0].Value[cx, cy];
              if (self.game.Data.MapObj[0].HexObj[cx, cy].LandscapeType == self.LOWMOUNTAIN)
                self.game.EditObj.TempValue2[0].Value[cx, cy] =  Math.Round( (self.game.EditObj.TempValue2[0].Value[cx, cy] + 20000) / 2.0);
              else if (self.game.Data.MapObj[0].HexObj[cx, cy].LandscapeType == self.HIGHMOUNTAIN)
                self.game.EditObj.TempValue2[0].Value[cx, cy] =  Math.Round( (self.game.EditObj.TempValue2[0].Value[cx, cy] + 50000) / 2.0);
            }
          }
        }
        let mut mapWidth3: i32 = self.game.Data.MapObj[0].MapWidth;
        for (let mut index11: i32 = 0; index11 <= mapWidth3; index11 += 1)
        {
          let mut mapHeight: i32 = self.game.Data.MapObj[0].MapHeight;
          for (let mut index12: i32 = 0; index12 <= mapHeight; index12 += 1)
            self.game.EditObj.TempValue[0].Value[index11, index12] = self.game.EditObj.TempValue2[0].Value[index11, index12];
        }
      }
      while (num3 < 50);
      let mut mapWidth4: i32 = self.game.Data.MapObj[0].MapWidth;
      for (let mut index13: i32 = 0; index13 <= mapWidth4; index13 += 1)
      {
        let mut mapHeight: i32 = self.game.Data.MapObj[0].MapHeight;
        for (let mut index14: i32 = 0; index14 <= mapHeight; index14 += 1)
        {
          if ( self.game.Data.RuleVar[450] == 1.0)
            self.game.EditObj.TempValue[0].Value[index13, index14] =  Math.Round(0.95 *  self.game.EditObj.TempValue[0].Value[index13, index14] + 0.1 *  self.game.EditObj.TempValue[0].Value[index13, index14] *  VBMath.Rnd());
          if (self.game.Data.MapObj[0].HexObj[index13, index14].LandscapeType == self.WATER)
            self.game.EditObj.TempValue[0].Value[index13, index14] = 0;
          else if (self.game.EditObj.TempValue[0].Value[index13, index14] <= 25)
            self.game.EditObj.TempValue[0].Value[index13, index14] = 25;
        }
      }
      if (self.domirror != 0)
        return;
      let mut mapWidth5: i32 = self.game.Data.MapObj[0].MapWidth;
      for (let mut cx: i32 = 0; cx <= mapWidth5; cx += 1)
      {
        let mut mapHeight: i32 = self.game.Data.MapObj[0].MapHeight;
        for (let mut cy: i32 = 0; cy <= mapHeight; cy += 1)
        {
          if ( VBMath.Rnd() < 0.07)
          {
            if (self.game.EditObj.TempValue[0].Value[cx, cy] != 0)
            {
              let mut tfacing: i32 = 1;
              do
              {
                coordinate = self.game.HandyFunctionsObj.HexNeighbourSameMap(cx, cy, 0, tfacing);
                if (self.game.EditObj.TempValue[0].Value[coordinate.x, coordinate.y] != 0)
                {
                  if (coordinate.onmap)
                    self.game.EditObj.TempValue[0].Value[coordinate.x, coordinate.y] = Conversion.Int(self.game.EditObj.TempValue[0].Value[cx, cy]);
                  self.game.EditObj.TempValue[0].Value[cx, cy] = Conversion.Int(self.game.EditObj.TempValue[0].Value[cx, cy]);
                }
                tfacing += 1;
              }
              while (tfacing <= 6);
            }
          }
          else
            self.game.EditObj.TempValue[0].Value[cx, cy] =  Math.Round(Conversion.Int( VBMath.Rnd() * 0.1 *  self.game.EditObj.TempValue[0].Value[cx, cy]) + Conversion.Int(0.95 *  self.game.EditObj.TempValue[0].Value[cx, cy]));
        }
      }
    }

    pub fn MakeLandBlob(x: i32, y: i32, sizy: i32)
    {
      let mut num1: i32 = 0;
      self.game.Data.MapObj[0].HexObj[x, y].LandscapeType = self.GRASS;
      this += 1.landcur;
      if ( VBMath.Rnd() * 100.0 <  self.opt6v)
        num1 =  VBMath.Rnd() >= 0.3 ? ( VBMath.Rnd() >= 0.5 ? 3 : 2) : 1;
      num2: i32;
      do
      {
        num2 = 0;
        num3: i32;
        num3 += 1;
        let mut mapWidth: i32 = self.game.Data.MapObj[0].MapWidth;
        for (let mut x2: i32 = 0; x2 <= mapWidth; x2 += 1)
        {
          let mut mapHeight: i32 = self.game.Data.MapObj[0].MapHeight;
          for (let mut y2: i32 = 0; y2 <= mapHeight; y2 += 1)
          {
            if (self.game.HandyFunctionsObj.Distance(x, y, 0, x2, y2, 0) == num3)
            {
              float num4 = VBMath.Rnd() * 0.5f;
              if ( Conversion.Int( ( VBMath.Rnd() * ( sizy *  num4) +  sizy * (1.0 -  num4))) >  (num3 * num3) && self.game.Data.MapObj[0].HexObj[x2, y2].LandscapeType == self.WATER | self.opt4v == 100 | self.WATER == -1)
              {
                switch (num1)
                {
                  case 1:
                    num2 = 1;
                    if ( VBMath.Rnd() < 0.8)
                    {
                      self.game.Data.MapObj[0].HexObj[x2, y2].LandscapeType = self.HEAVYFOREST;
                      break;
                    }
                    if ( VBMath.Rnd() < 0.4)
                    {
                      self.game.Data.MapObj[0].HexObj[x2, y2].LandscapeType = self.LIGHTFOREST;
                      break;
                    }
                    self.game.Data.MapObj[0].HexObj[x2, y2].LandscapeType = self.GRASS;
                    break;
                  case 2:
                    num2 = 1;
                    if ( VBMath.Rnd() < 0.6)
                    {
                      self.game.Data.MapObj[0].HexObj[x2, y2].LandscapeType = self.LIGHTFOREST;
                      break;
                    }
                    if ( VBMath.Rnd() < 0.6)
                    {
                      self.game.Data.MapObj[0].HexObj[x2, y2].LandscapeType = self.HEAVYFOREST;
                      break;
                    }
                    self.game.Data.MapObj[0].HexObj[x2, y2].LandscapeType = self.GRASS;
                    break;
                  case 3:
                    num2 = 1;
                    if ( VBMath.Rnd() < 0.6)
                    {
                      self.game.Data.MapObj[0].HexObj[x2, y2].LandscapeType = self.LIGHTFOREST;
                      break;
                    }
                    if ( VBMath.Rnd() < 0.1)
                    {
                      self.game.Data.MapObj[0].HexObj[x2, y2].LandscapeType = self.LIGHTFOREST;
                      break;
                    }
                    self.game.Data.MapObj[0].HexObj[x2, y2].LandscapeType = self.GRASS;
                    break;
                  default:
                    num2 = 1;
                    self.game.Data.MapObj[0].HexObj[x2, y2].LandscapeType = self.GRASS;
                    break;
                }
                this += 1.landcur;
              }
            }
          }
        }
      }
      while (num2 == 1);
    }

    pub fn MakeMountainRange(x: i32, y: i32, x2: i32, y2: i32)
    {
      let mut mapWidth1: i32 = self.game.Data.MapObj[0].MapWidth;
      Coordinate coordinate;
      for (let mut cx: i32 = 0; cx <= mapWidth1; cx += 1)
      {
        let mut mapHeight: i32 = self.game.Data.MapObj[0].MapHeight;
        for (let mut cy: i32 = 0; cy <= mapHeight; cy += 1)
        {
          if (self.game.Data.MapObj[0].HexObj[cx, cy].LandscapeType == self.GRASS && cx >= x & cy >= y & cx <= x2 & cy <= y2)
          {
            let mut num: i32 = 1;
            let mut tfacing: i32 = 1;
            do
            {
              coordinate = self.game.HandyFunctionsObj.HexNeighbourSameMap(cx, cy, 0, tfacing);
              if (coordinate.onmap && self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].LandscapeType == self.WATER)
                num = 0;
              tfacing += 1;
            }
            while (tfacing <= 6);
            if (num == 1)
            {
              if ( VBMath.Rnd() < 0.66)
              {
                self.game.Data.MapObj[0].HexObj[cx, cy].LandscapeType = self.HIGHMOUNTAIN;
                this += 1.mountaincur;
              }
              else
              {
                self.game.Data.MapObj[0].HexObj[cx, cy].LandscapeType = self.LOWMOUNTAIN;
                this += 1.mountaincur;
              }
            }
          }
        }
      }
      num1: i32;
      num1 += 1;
      let mut mapWidth2: i32 = self.game.Data.MapObj[0].MapWidth;
      for (let mut cx: i32 = 0; cx <= mapWidth2; cx += 1)
      {
        let mut mapHeight: i32 = self.game.Data.MapObj[0].MapHeight;
        for (let mut cy: i32 = 0; cy <= mapHeight; cy += 1)
        {
          if (self.game.Data.MapObj[0].HexObj[cx, cy].LandscapeType == self.GRASS && cx >= x - 1 & cy >= y - 1 & cx <= x2 + 1 & cy <= y2 + 1)
          {
            let mut num2: i32 = 0;
            let mut tfacing: i32 = 1;
            do
            {
              coordinate = self.game.HandyFunctionsObj.HexNeighbourSameMap(cx, cy, 0, tfacing);
              if (coordinate.onmap && self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].LandscapeType == self.HIGHMOUNTAIN)
                num2 = 1;
              tfacing += 1;
            }
            while (tfacing <= 6);
            if (num2 == 1 &&  VBMath.Rnd() > 0.5)
            {
              self.game.Data.MapObj[0].HexObj[cx, cy].LandscapeType = self.LOWMOUNTAIN;
              this += 1.mountaincur;
            }
          }
        }
      }
    }

    pub fn FinalizeLadder()
    {
      numArray: Vec<i32> = new int[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      self.game.Data.AddPeople();
      self.game.Data.PeopleObj[self.game.Data.PeopleCounter].PeopleGroup = 1;
      self.game.Data.PeopleObj[self.game.Data.PeopleCounter].Name = "2nd Player";
      self.game.Data.RegimeObj[1].People = 1;
      self.game.Data.TempString[200] = "Universals";
      self.game.Data.TempString[201] = "2nd Player";
      let mut mapWidth: i32 = self.game.Data.MapObj[0].MapWidth;
      for (let mut x: i32 = 0; x <= mapWidth; x += 1)
      {
        let mut mapHeight: i32 = self.game.Data.MapObj[0].MapHeight;
        for (let mut y: i32 = 0; y <= mapHeight; y += 1)
        {
          let mut regime: i32 = self.game.Data.MapObj[0].HexObj[x, y].Regime;
          if (regime > -1)
          {
            let mut location: i32 = self.game.Data.MapObj[0].HexObj[x, y].Location;
            if (location > -1)
            {
              self.game.Data.LocObj[location].HQ = regime;
              self.game.Data.LocObj[location].ProdPercent[0] = 100;
              self.game.Data.LocObj[location].Production[0] = 1;
              let mut unr: i32 = self.game.Data.AddUnit(x, y, 0);
              self.game.Data.UnitObj[unr].Name = "Garrison Unit";
              self.game.Data.UnitObj[unr].Regime = regime;
              self.game.Data.UnitObj[unr].Supply = 80;
              self.game.Data.UnitObj[unr].SOSupReqPercent = 100;
              self.game.Data.UnitObj[unr].IsHQ = false;
              self.game.Data.UnitObj[unr].HQ = regime;
              let mut index: i32 = self.game.Data.AddSF(unr);
              self.game.Data.SFObj[index].Type = 0;
              self.game.Data.SFObj[index].Qty = 20;
              self.game.Data.SFObj[index].Rdn = 100;
              self.game.Data.SFObj[index].People = 0;
              self.game.Data.SFObj[index].Xp = 25;
              self.game.Data.SFObj[index].Mor = 50;
            }
          }
        }
      }
    }

    pub fn FinalizeLadderPre()
    {
      numArray: Vec<i32> = new int[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      let mut mapWidth1: i32 = self.game.Data.MapObj[0].MapWidth;
      for (let mut index1: i32 = 0; index1 <= mapWidth1; index1 += 1)
      {
        let mut mapHeight: i32 = self.game.Data.MapObj[0].MapHeight;
        for (let mut index2: i32 = 0; index2 <= mapHeight; index2 += 1)
          numArray[index1, index2] = -1;
      }
      let mut num1: i32 = 1;
      do
      {
        let mut mapWidth2: i32 = self.game.Data.MapObj[0].MapWidth;
        for (let mut index3: i32 = 0; index3 <= mapWidth2; index3 += 1)
        {
          let mut mapHeight: i32 = self.game.Data.MapObj[0].MapHeight;
          for (let mut index4: i32 = 0; index4 <= mapHeight; index4 += 1)
          {
            if (self.game.Data.MapObj[0].HexObj[index3, index4].Regime == num1)
              numArray[index3, index4] = 0;
          }
        }
        num1 += -1;
      }
      while (num1 >= 0);
      let mut num2: i32 = 0;
      do
      {
        let mut num3: i32 = 1;
        do
        {
          let mut mapWidth3: i32 = self.game.Data.MapObj[0].MapWidth;
          for (let mut cx: i32 = 0; cx <= mapWidth3; cx += 1)
          {
            let mut mapHeight: i32 = self.game.Data.MapObj[0].MapHeight;
            for (let mut cy: i32 = 0; cy <= mapHeight; cy += 1)
            {
              if (self.game.Data.MapObj[0].HexObj[cx, cy].Regime == num3 & numArray[cx, cy] == num2)
              {
                let mut tfacing: i32 = 1;
                do
                {
                  Coordinate coordinate = self.game.HandyFunctionsObj.HexNeighbourSameMap(cx, cy, 0, tfacing);
                  if (coordinate.onmap & numArray[coordinate.x, coordinate.y] == -1)
                  {
                    if (!self.game.Data.LandscapeTypeObj[self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].LandscapeType].IsSea && self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].Regime == -1)
                      self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].Regime = num3;
                    numArray[coordinate.x, coordinate.y] = num2 + 1;
                  }
                  tfacing += 1;
                }
                while (tfacing <= 6);
              }
            }
          }
          num3 += -1;
        }
        while (num3 >= 0);
        num2 += 1;
      }
      while (num2 <= 20);
    }
  }
}
